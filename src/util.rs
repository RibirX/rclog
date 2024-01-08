use std::{cell::RefCell, vec};

// Update the "comrak --help" text in Comrak's own README.
use comrak::{
  arena_tree::Node,
  format_commonmark,
  nodes::{Ast, AstNode, NodeCode, NodeHeading, NodeValue},
  parse_document, Arena, Options,
};
use semver::{Version, VersionReq};

/// Pick the changelog content of the given version
pub fn extract_content(
  version: &Version,
  changelog: &str,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
  let arena = Arena::new();
  let doc = parse_document(&arena, changelog, &Options::default());

  let mut content = vec![];
  let mut in_version_heading = false;
  for node in doc.children() {
    if !in_version_heading {
      in_version_heading = version_title(node).map_or(false, |v| &v == version);
      continue;
    } else if matches!(
        node.data.borrow().value,
        NodeValue::Heading(h) if h.level <= 2
    ) {
      break;
    }
    format_commonmark(node, &<_>::default(), &mut content)?;
  }
  if content.is_empty() {
    Err("No changelog content found".into())
  } else {
    Ok(String::from_utf8(content)?)
  }
}

/// Merge the pre-release version changelog to the more stable version
/// For example, if your content contains the changelog of 0.1.1-alpha.1 and
/// 0.1.1-alpha.2, the more stable version you provide is 0.1.2.beta.1,
/// then all changelog of 0.1.1-alpha.1 and 0.1.1-alpha.2 will be merged to
/// 0.1.2.beta.1
pub fn merge_pre_release_changelogs(
  version: &Version,
  content: &str,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
  let arena = Arena::new();
  let doc = parse_document(&arena, content, &Options::default());

  struct MergePart<'a> {
    title: String,
    content: Vec<&'a Node<'a, RefCell<Ast>>>,
  }

  let mut merge_parts: Vec<MergePart<'_>> = vec![];
  let options = <_>::default();
  let mut merging_part_name = None;
  let mut in_version_heading = false;
  let mut nodes = vec![];
  let mut version_idx = -1;
  let mut no_subtitle_nodes = vec![];
  for node in doc.children() {
    if let Some(v) = version_title(node) {
      // version title matched
      if VersionReq::parse(&v.to_string())?.matches(version) {
        in_version_heading = true;
        // only keep first version heading, other version headings will be removed
        if &v == version {
          if version_idx != -1 {
            return Err("Multiple version headings".into());
          }
          version_idx = nodes.len() as i32;
          nodes.push(node);
        }
        continue;
      }
    }

    if matches!(
        node.data.borrow().value,
        NodeValue::Heading(h) if h.level <= 2
    ) {
      in_version_heading = false;
    }

    if !in_version_heading {
      nodes.push(node)
    } else {
      if matches!(
          node.data.borrow().value,
          NodeValue::Heading(h) if h.level == 3
      ) {
        let mut content = vec![];
        format_commonmark(node, &options, &mut content)?;
        let title = String::from_utf8(content)?;
        merging_part_name = Some(title.clone());
        if !merge_parts.iter().any(|p| p.title == title) {
          merge_parts.push(MergePart { title, content: vec![] });
        }
        continue;
      } else if matches!(
          node.data.borrow().value,
          NodeValue::Heading(h) if h.level < 3
      ) {
        merging_part_name = None;
      }
      if let Some(name) = merging_part_name.as_ref() {
        let part = merge_parts.iter_mut().find(|p| &p.title == name).unwrap();
        part.content.push(node);
      } else {
        no_subtitle_nodes.push(node);
      }
    }
  }

  if version_idx == -1 {
    return Err("No version heading found".into());
  }

  let mut content = vec![];

  for (idx, node) in nodes.iter().enumerate() {
    format_commonmark(node, &options, &mut content)?;
    content.push(b'\n');
    if idx == version_idx as usize {
      for node in no_subtitle_nodes.iter() {
        format_commonmark(node, &options, &mut content)?;
        content.push(b'\n');
      }
      for part in merge_parts.iter() {
        content.extend(part.title.as_bytes());
        content.push(b'\n');
        for node in part.content.iter().rev() {
          format_commonmark(node, &options, &mut content)?;
          content.push(b'\n');
        }
      }
    }
  }

  // pop the last '\n'
  content.pop();
  if content.is_empty() {
    Err("No changelog content found".into())
  } else {
    Ok(String::from_utf8(content)?)
  }
}

fn pick_version<'a>(node: &'a AstNode<'a>) -> Option<Version> {
  match node.data.borrow().value {
    NodeValue::Text(ref literal) | NodeValue::Code(NodeCode { ref literal, .. }) => {
      let ver_str = literal.split_whitespace().next()?;
      Version::parse(ver_str).ok()
    }
    _ => {
      for n in node.children() {
        if let Some(v) = pick_version(n) {
          return Some(v);
        }
      }
      None
    }
  }
}

fn version_title<'a>(node: &'a AstNode<'a>) -> Option<Version> {
  if let NodeValue::Heading(NodeHeading { level: 2, .. }) = node.data.borrow().value {
    pick_version(node)
  } else {
    None
  }
}

#[test]
fn pick_content() {
  let content = "# Changelog

## [0.1.3] - 2024-01-03

version 0.1.3

## [0.1.2] - 2024-01-02

### Features

- version [0.1.2]

## [0.1.1] - 2024-01-01

version 0.1.1

[0.1.2]: ribir.org
";
  let v0_1_2 = Version::parse("0.1.2").unwrap();
  let content = extract_content(&v0_1_2, content).unwrap();
  assert_eq!(content, "### Features\n- version [0.1.2](ribir.org)\n");
}

#[test]
fn merge() {
  let changelog = "# Changelog

## [0.1.0] - 2024-01-03

### Features

- stable version bump

## [0.1.0-alpha.2] - 2024-01-02

### Features

- bump version alpha.2

## [0.1.0-alpha.1] - 2024-01-01

### Features

- bump version alpha.1

[0.1.0]: ribir.org
[0.1.0-alpha.2]: ribir.org
[0.1.0-alpha.1]: ribir.org
";

  let version = Version::parse("0.1.0").unwrap();
  let new_changelog = merge_pre_release_changelogs(&version, changelog).unwrap();
  assert_eq!(
    "# Changelog

## [0.1.0](ribir.org) - 2024-01-03

### Features

- bump version alpha.1

- bump version alpha.2

- stable version bump
",
    new_changelog,
  )
}

#[test]
fn fix_merge_wrong_version() {
  let changelog = "# Changelog

  ## Unreleased 
  
  The version not released yet.
  
  ## 0.1.1-alpha.1 - 2024-01-02
  
  But a pre-release version is released.
  ";

  let version = Version::parse("0.1.1").unwrap();
  let new_changelog = merge_pre_release_changelogs(&version, changelog);
  assert!(new_changelog.is_err())
}
