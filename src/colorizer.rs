use std::{cmp::Ordering, collections::HashMap, ops::Range};

use crate::rules::rule::Node;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Color(pub [u8; 4]);

impl Color {
    pub const WHITE: Color = Color::from_rgb(255, 255, 255);

    #[inline(always)]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 255])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Section {
    pub range: Range<usize>,
    pub color: Color,
}

pub fn colorize(length: usize, colors: &HashMap<&str, Color>, nodes: &Vec<Node>) -> Vec<Section> {
    let mut ranges = vec![];
    for node in nodes.iter() {
        get_color(colors, node, &mut ranges);
    }

    return merge(
        Section {
            range: std::ops::Range {
                start: 0,
                end: length,
            },
            color: Color::WHITE,
        },
        &mut ranges,
    );
}

fn get_color(colors: &HashMap<&str, Color>, node: &Node, vec: &mut Vec<Section>) {
    if let Some(color) = colors.get(node.rulename) {
        let mut child_ranges = vec![];
        for node in node.subnodes.iter() {
            get_color(colors, node, &mut child_ranges);
        }
        if child_ranges.len() > 0 {
            vec.append(&mut merge(
                Section {
                    range: node.range.clone(),
                    color: *color,
                },
                &mut child_ranges,
            ))
        } else {
            vec.push(Section {
                range: node.range.clone(),
                color: *color,
            });
        }
    }
    for node in node.subnodes.iter() {
        get_color(colors, node, vec);
    }
}

fn merge(state_range: Section, ranges: &mut Vec<Section>) -> Vec<Section> {
    let mut new_ranges = vec![];

    ranges.sort_by(|a, b| {
        if a.range.start > b.range.start {
            return Ordering::Greater;
        } else if a.range.start < b.range.start {
            return Ordering::Less;
        }
        return Ordering::Equal;
    });

    let mut i = state_range.range.start;
    for range in ranges.iter() {
        if range.range.start > i {
            new_ranges.push(Section {
                range: Range {
                    start: i,
                    end: range.range.start,
                },
                color: state_range.color,
            });
            i = range.range.start;
        }
        if range.range.start == i && range.range.start <= range.range.end {
            new_ranges.push(Section {
                range: range.range.clone(),
                color: range.color,
            });
            i += range.range.len();
        }
    }

    if i < state_range.range.end {
        new_ranges.push(Section {
            range: Range {
                start: i,
                end: state_range.range.end,
            },
            color: state_range.color,
        });
    }

    return new_ranges;
}
