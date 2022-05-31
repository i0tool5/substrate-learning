use frame_support::dispatch::Weight;

pub trait WeightInfo {
    fn create_todo(l: usize) -> Weight;
}