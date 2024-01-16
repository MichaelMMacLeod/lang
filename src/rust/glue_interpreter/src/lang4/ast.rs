pub struct Ast {}

// for x (a x .. b) -> (x ..)
// ({} ..{m})
//
// for x (a (b x) .. c) -> (x ..)
// ({1} ..{m})
//
// for x (a a2 (b (c x) ..) d) -> (x ..)
// ({1} ..{2 m})
//
// for x (a a2 (b (c x ..) ..) d) -> (x .. ..)
// ({} ..{m2} ..{m1})