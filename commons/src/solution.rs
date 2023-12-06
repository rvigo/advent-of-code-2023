use crate::answer::Answer;

pub trait Solution {
    fn first_part(&self, input: &str) -> Answer;

    fn second_part(&self, input: &str) -> Answer;
}
