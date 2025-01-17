use crate::regex::{
    byteclass::{ByteClass, ByteClassId},
    stateid::StateId,
};
use indexmap::IndexSet;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Default)]
pub struct DfaState<A> {
    // A list of maximum length 256, but usually much shorter.
    // It lists all the states self is connected to. None means
    // a none existent state.
    table: Vec<Option<StateId>>,

    // A byteclass is a [u8; 256], and says how to move from
    // one state to another. If lets say dfa[self.class][c] == 5, then in
    // terms of a dfa pictorial representation, we have a edge going from
    // self to self.table[c], with the value 5 associated with that edge.
    class: ByteClassId,

    associations: HashSet<A>,
}

impl<A> DfaState<A> {
    fn empty() -> Self {
        Self {
            table: vec![None],
            class: ByteClassId(0),
            associations: HashSet::new(),
        }
    }
}

impl<A: Eq + std::hash::Hash> DfaState<A> {
    #[cfg(test)]
    pub fn is_associated_with(&self, value: &A) -> bool {
        self.associations.contains(value)
    }
}

#[derive(Debug, Clone)]
pub struct DFA<A> {
    /// The states are the nodes of the DFA.
    pub(crate) states: Vec<DfaState<A>>,
    ends: Vec<StateId>,
    pub(crate) transitions: IndexSet<ByteClass>,
}

impl<A> Default for DFA<A> {
    fn default() -> Self {
        Self {
            states: vec![],
            ends: vec![],
            transitions: IndexSet::new(),
        }
    }
}

impl<A: std::hash::Hash + Eq + Clone> DFA<A> {
    pub fn new() -> Self {
        Self {
            states: vec![],
            ends: vec![],
            transitions: IndexSet::new(),
        }
    }

    pub fn rough_size_bytes(&self) -> u64 {
        self.transitions.len() as u64 * 256
    }

    pub fn number_of_states(&self) -> usize {
        self.states.len()
    }

    pub fn dedup_ends(&mut self) {
        self.ends.dedup()
    }

    pub fn push_end(&mut self, end: StateId) {
        self.ends.push(end);
    }

    /// Create a new empty state and returns its id.
    pub(crate) fn push_state(&mut self) -> StateId {
        let id = StateId::of(self.states.len());
        self.states.push(DfaState::empty());
        id
    }

    pub(crate) fn push_class(&mut self, class: ByteClass) -> ByteClassId {
        if let Some(id) = self.transitions.get_index_of(&class) {
            ByteClassId(id as u16)
        } else {
            let id = ByteClassId(self.transitions.len() as u16);
            self.transitions.insert(class);
            id
        }
    }

    pub(crate) fn set_transitions<I>(&mut self, id: StateId, transitions: I)
    where
        I: IntoIterator<Item = Option<StateId>>,
    {
        let mut table = vec![];
        let mut seen = IndexSet::new();
        let mut class = ByteClass::empty();
        for (b, id) in transitions.into_iter().enumerate() {
            if let Some(i) = seen.get_index_of(&id) {
                class[b as u8] = i as u8;
            } else {
                class[b as u8] = seen.len() as u8;
                seen.insert(id);
                table.push(id);
            }
        }

        self.states[id.0 as usize].table = table;
        let class_id = self.push_class(class);
        self.states[id.0 as usize].class = class_id;
    }

    pub(crate) fn associate(&mut self, id: StateId, assosiated_values: HashSet<A>) {
        self.states[id.0 as usize]
            .associations
            .extend(assosiated_values);
    }

    pub fn associations(&self, id: StateId) -> Vec<A> {
        let state = &self[id];
        state.associations.iter().cloned().collect::<Vec<A>>()
    }

    pub fn find<I: AsRef<[u8]>>(&self, input: I) -> Result<StateId, Option<StateId>> {
        if self.states.is_empty() {
            return Err(None);
        }

        let mut current = StateId::of(0);
        for b in input.as_ref() {
            if let Some(next) = self[(current, *b)] {
                current = next;
            } else {
                return Err(Some(current));
            }
        }
        if self.ends.contains(&current) {
            Ok(current)
        } else {
            Err(Some(current))
        }
    }
}

impl<A> Index<u8> for DfaState<A> {
    type Output = Option<StateId>;
    fn index(&self, index: u8) -> &Self::Output {
        &self.table[index as usize]
    }
}

impl<A> Index<StateId> for DFA<A> {
    type Output = DfaState<A>;
    fn index(&self, index: StateId) -> &Self::Output {
        &self.states[index.0 as usize]
    }
}

impl<A> IndexMut<StateId> for DFA<A> {
    fn index_mut(&mut self, index: StateId) -> &mut Self::Output {
        &mut self.states[index.0 as usize]
    }
}

impl<A> Index<ByteClassId> for DFA<A> {
    type Output = ByteClass;
    fn index(&self, ByteClassId(index): ByteClassId) -> &Self::Output {
        &self.transitions[index as usize]
    }
}

impl<A> Index<(StateId, u8)> for DFA<A> {
    type Output = Option<StateId>;

    fn index(&self, (id, b): (StateId, u8)) -> &Self::Output {
        let state = &self[id];
        &state[self[state.class.clone()][b]]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::regex::nfa::NFA;

    #[test]
    fn empty() {
        let empty = DFA::<usize>::new();
        let x = empty.find("");
        assert!(x.is_err());
        assert!(x.unwrap_err().is_none());
    }

    #[test]
    fn literal() {
        let nfa = NFA::<usize>::literal("hello");
        let dfa: DFA<usize> = nfa.into();
        assert!(dfa.find("hello").is_ok());
        assert!(dfa.find("ello").is_err());
        assert!(dfa.find("hhello").is_err());
        assert!(dfa.find("helloo").is_err());
        assert!(dfa.find("helo").is_err());
        assert!(dfa.find("hxllo").is_err());
        assert!(dfa.find("hell").is_err());
        assert!(dfa.find("helln").is_err());
    }

    #[test]
    fn empty_literal() {
        let nfa = NFA::<usize>::literal("");
        let dfa: DFA<usize> = nfa.into();
        assert!(dfa.find("").is_ok());
        assert!(dfa.find(" ").is_err());
    }

    #[test]
    fn simple_or() {
        let nfa = NFA::<usize>::literal("a");
        let nfb = NFA::<usize>::literal("b");

        let dfa: DFA<usize> = nfa.or(nfb).unwrap().into();

        assert!(dfa.find("a").is_ok());
        assert!(dfa.find("b").is_ok());
        assert!(dfa.find("c").is_err());
        assert!(dfa.find("").is_err());
    }

    #[test]
    fn simple_or_eq() {
        let nfa = NFA::<usize>::literal("a");
        let nfb = NFA::<usize>::literal("a");

        let dfa: DFA<usize> = nfa.or(nfb).unwrap().into();

        assert!(dfa.find("a").is_ok());
        assert!(dfa.find("b").is_err());
        assert!(dfa.find("").is_err());
    }

    #[test]
    fn simple_or_empty() {
        let nfa = NFA::<usize>::literal("a");
        let nfb = NFA::<usize>::literal("");

        let dfa: DFA<usize> = nfa.or(nfb).unwrap().into();

        assert!(dfa.find("a").is_ok());
        assert!(dfa.find("b").is_err());
        assert!(dfa.find("").is_ok());
    }

    #[test]
    fn empty_or_empty() {
        let nfa = NFA::<usize>::literal("");
        let nfb = NFA::<usize>::literal("");

        let dfa: DFA<usize> = nfa.or(nfb).unwrap().into();

        assert!(dfa.find("a").is_err());
        assert!(dfa.find("b").is_err());
        assert!(dfa.find("").is_ok());
    }

    #[test]
    fn empty_or_simple() {
        let nfa = NFA::<usize>::literal("");
        let nfb = NFA::<usize>::literal("a");

        let dfa: DFA<usize> = nfa.or(nfb).unwrap().into();

        assert!(dfa.find("a").is_ok());
        assert!(dfa.find("b").is_err());
        assert!(dfa.find("").is_ok());
    }
}
