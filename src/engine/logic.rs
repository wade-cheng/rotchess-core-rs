// //! A chess engine for rotchess AI.
// //!
// //! We're implementing
// //!
// //! - negamax
// //! - alpha-beta pruning
// //! - quiescence search

// pub type Score = f32;

// /// Find the best move.
// pub fn best_move<M>() -> M {
//     /// remaining depth to search for moves in plies.
//     const DEPTH: usize = 4;
//     todo!()
// }

// /// Returns the score of the best move.
// ///
// /// Logic uses negamax with alpha-beta pruning.
// fn negamax_ab(
//     eval: impl Fn(Score, Score) -> Score,
//     moves: impl FnMut()
//     alpha: Score,
//     beta: Score,
//     depth: usize,
// ) -> Score {
//     if depth == 0 {
//         return eval(alpha, beta);
//     }

//     let best_val: Score = Score::NEG_INFINITY;
//     for move_ in moves() {
//         todo!()
//     }
//     todo!()
// }
