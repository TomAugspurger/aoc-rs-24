/*

Given a target code like 029A

Figure out the shortest sequence of button presses to get there.

I think the length of the intermediate button presses is irrelevant;
they just need to be valid (not panic, get the right answer).
*/

pub enum DPad {
    A,
    U,
    D,
    L,
    R,
}

pub enum NumPad {
    A,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

/*
Get the shortest sequence of button presses on the
dpad to produce the target code. Facts:

1. Each intermediate press (from us or a robot) starts on "A".
    - A->U is 1 + 1 + 1 (L + R + P)
    - A->R is 1
    - A->D is 2
    - A->L is 3
2.

--D-LL---A
<vA<AA>>^A                   |vAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
v<<A>>^A                     |<A>AvA<^AA>A<vAAA>^A
<A                           |^A>^^AvvvA
0                            |29A
*/

impl NumPad {
    pub fn move_(&self, to: &NumPad) -> Vec<DPad> {

        let mut result = Vec::new();

        // // Danger zones:
        // // - "0" can't move left
        // // - "1" can't move down
        // // Facts:
        // // We always start from "A"? No. But we end on "A"? Yes.

        // match (self, to) {
        //     // A->*
        //     (NumPad::A, NumPad::_0) => {
        //         result.push(DPad::L);
        //     }
        //     (NumPad::A, NumPad::_3) => {
        //         result.push(DPad::U);
        //     }
        //     (NumPad::A, NumPad::_2) => {
        //         result.extend(self.move_(&NumPad::_3));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::A, NumPad::_1) => {
        //         result.extend(self.move_(&NumPad::_2));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::A, NumPad::_6) => {
        //         result.extend(self.move_(&NumPad::_3));
        //         result.push(DPad::U);
        //     }
        //     (NumPad::A, NumPad::_5) => {
        //         result.extend(self.move_(&NumPad::_6));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::A, NumPad::_4) => {
        //         result.extend(self.move_(&NumPad::_5));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::A, NumPad::_9) => {
        //         result.extend(self.move_(&NumPad::_6));
        //         result.push(DPad::U);
        //     }
        //     (NumPad::A, NumPad::_8) => {
        //         result.extend(self.move_(&NumPad::_9));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::A, NumPad::_7) => {
        //         result.extend(self.move_(&NumPad::_9));
        //         result.push(DPad::L);
        //     }
        //     // B->*
        //     (NumPad::_0, NumPad::_2) => {
        //         result.push(DPad::U);
        //     }
        //     (NumPad::_0, NumPad::_1) => {
        //         result.extend(self.move_(&NumPad::_2));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::_0, NumPad::_3) => {
        //         result.extend(self.move_(&NumPad::_2));
        //         result.push(DPad::R);
        //     }
        //     (NumPad::_0, NumPad::_5) => {
        //         result.push(DPad::U);
        //         result.push(DPad::U);
        //     }
        //     (NumPad::_0, NumPad::_4) => {
        //         result.extend(self.move_(&NumPad::_5));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::_0, NumPad::_6) => {
        //         result.extend(self.move_(&NumPad::_5));
        //         result.push(DPad::R);
        //     }
        //     (NumPad::_0, NumPad::_8) => {
        //         result.extend([DPad::U, DPad::U, DPad::U]);
        //     }
        //     (NumPad::_0, NumPad::_7) => {
        //         result.extend(self.move_(&NumPad::_8));
        //         result.push(DPad::L);
        //     }
        //     (NumPad::_0, NumPad::_9) => {
        //         result.extend(self.move_(&NumPad::_8));
        //         result.push(DPad::R);

        //     }


        //     _ => {
        //         result.extend(to.move_(self));
        //     }
        // }

        result
    }

    pub fn press(&self, to: &NumPad) -> Vec<DPad> {
        let mut result = self.move_(to);
        result.push(DPad::A);
        result
    }
}

pub fn solve(code: &str) -> Vec<DPad> {
    // What state do we need to keep track of? Each robot
    // has a current position.
    // We have 3 robots:
    // 1. The robot pushing the numpad, controlling the door
    // 2. The robot pushing the dpad, controlling robot 1.
    // 3. The robot pushing the dpad, controlling robot 2.

    // We only really care about *our* number of presses.

    let mut p1 = NumPad::A;
    let mut p2 = DPad::A;
    let mut p3 = DPad::A;
    let mut reuslt: Vec<DPad> = Vec::new();
    let mut pressed = String::new();

    for c in code.chars() {
        // What
        // r1
    }

    todo!()
}

pub fn main(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
029A
980A
179A
456A
379A";

    #[test]
    fn test_example_1() {
        let result = main(INPUT);
        assert_eq!(result, 126384);
    }
}
