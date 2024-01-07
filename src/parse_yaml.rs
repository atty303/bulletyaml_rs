use crate::errors::{ParseError, ParseErrorPos};
use crate::tree::{
    BulletML, BulletMLExpression, BulletMLNode, BulletMLType, DirectionType, HVType, SpeedType,
};
use indextree::{Arena, NodeId};
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};


/// BulletML parser.
pub struct BulletYAMLParser {
    arena: Arena<BulletMLNode>,
    bullet_refs: HashMap<String, NodeId>,
    action_refs: HashMap<String, NodeId>,
    fire_refs: HashMap<String, NodeId>,
    expr_parser: fasteval::Parser,
    expr_slab: fasteval::Slab,
}

impl BulletYAMLParser {
    /// Creates a new parser with default capacities.
    ///
    /// Pay attention to the fact that the capacity of the expression parser cannot grow due to
    /// `fasteval::Slab` implementation. If you need a higher capacity, refer to the
    /// [with_capacities](#method.with_capacities) constructor.
    pub fn new() -> Self {
        BulletYAMLParser {
            arena: Arena::new(),
            bullet_refs: HashMap::new(),
            action_refs: HashMap::new(),
            fire_refs: HashMap::new(),
            expr_parser: fasteval::Parser::new(),
            expr_slab: fasteval::Slab::new(),
        }
    }

    /// Creates a new parser with custom capacities.
    ///
    /// `refs_capacity` is the initial capacity of references containers which can grow on demand.
    ///
    /// `expr_capacity` is the capacity of the expression parser which cannot grow. In order to
    /// mitigate that limitation, the internal of this crate handle float literals without the help
    /// of the expression parser.
    pub fn with_capacities(refs_capacity: usize, expr_capacity: usize) -> Self {
        BulletYAMLParser {
            arena: Arena::new(),
            bullet_refs: HashMap::with_capacity(refs_capacity),
            action_refs: HashMap::with_capacity(refs_capacity),
            fire_refs: HashMap::with_capacity(refs_capacity),
            expr_parser: fasteval::Parser::new(),
            expr_slab: fasteval::Slab::with_capacity(expr_capacity),
        }
    }

    /// Parses an input XML document and transforms it into a [BulletML](../struct.BulletML.html)
    /// structure to be used by a [Runner](../struct.Runner.html).
    pub fn parse(mut self, s: &str) -> Result<BulletML, ParseError> {
        let doc = yaml_rust::YamlLoader::load_from_str(s)?;
        let root = doc.root_element();
        let root_name = root.tag_name();
        match root_name.name() {
            "bulletml" => {
                let root_id = self.parse_bulletml(root)?;
                Ok(BulletML {
                    arena: self.arena,
                    root: root_id,
                    bullet_refs: self.bullet_refs,
                    action_refs: self.action_refs,
                    fire_refs: self.fire_refs,
                    expr_slab: self.expr_slab,
                })
            }
            name => Err(ParseError::new_unexpected_element(
                name.to_string(),
                BulletMLParser::node_pos(&root),
            )),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BulletYAML {

}