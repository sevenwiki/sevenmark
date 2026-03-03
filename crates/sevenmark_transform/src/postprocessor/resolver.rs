use super::MediaResolutionMap;
use crate::text_utils::normalized_plain_text;
use crate::wiki::DocumentNamespace;
use sevenmark_ast::{Element, ResolvedDoc, ResolvedFile, ResolvedMediaInfo, Traversable};

pub(super) fn resolve_media_elements(elements: &mut [Element], resolved_map: &MediaResolutionMap) {
    for element in elements {
        resolve_media_recursive(element, resolved_map);
    }
}

fn resolve_media_recursive(element: &mut Element, resolved_map: &MediaResolutionMap) {
    if let Element::Media(media_elem) = element {
        let mut resolved = ResolvedMediaInfo::default();

        if let Some(file_param) = media_elem.parameters.get("file")
            && let Some(title) = normalized_plain_text(&file_param.value)
        {
            let key = (DocumentNamespace::File, title);
            let (file_url, width, height, is_valid) = resolved_map
                .get(&key)
                .cloned()
                .unwrap_or((None, None, None, false));
            resolved.file = Some(ResolvedFile {
                url: file_url.unwrap_or_default(),
                is_valid,
                width: width.map(|w| w as u32),
                height: height.map(|h| h as u32),
            });
        }

        if let Some(doc_param) = media_elem.parameters.get("document")
            && let Some(title) = normalized_plain_text(&doc_param.value)
        {
            let key = (DocumentNamespace::Document, title.clone());
            let is_valid = resolved_map
                .get(&key)
                .map(|(_, _, _, valid)| *valid)
                .unwrap_or(false);
            resolved.document = Some(ResolvedDoc { title, is_valid });
        }

        if let Some(cat_param) = media_elem.parameters.get("category")
            && let Some(title) = normalized_plain_text(&cat_param.value)
        {
            let key = (DocumentNamespace::Category, title.clone());
            let is_valid = resolved_map
                .get(&key)
                .map(|(_, _, _, valid)| *valid)
                .unwrap_or(false);
            resolved.category = Some(ResolvedDoc { title, is_valid });
        }

        if let Some(user_param) = media_elem.parameters.get("user")
            && let Some(title) = normalized_plain_text(&user_param.value)
        {
            let key = (DocumentNamespace::User, title.clone());
            let is_valid = resolved_map
                .get(&key)
                .map(|(_, _, _, valid)| *valid)
                .unwrap_or(false);
            resolved.user = Some(ResolvedDoc { title, is_valid });
        }

        if let Some(url_param) = media_elem.parameters.get("url")
            && let Some(url) = normalized_plain_text(&url_param.value)
        {
            resolved.url = Some(url);
        }

        if resolved.file.is_some()
            || resolved.document.is_some()
            || resolved.category.is_some()
            || resolved.user.is_some()
            || resolved.url.is_some()
        {
            media_elem.resolved_info = Some(resolved);
        }
    }

    element.traverse_children(&mut |child| {
        resolve_media_recursive(child, resolved_map);
    });
}
