mod w03c;
use w03c::{Rgb, Hsv, Color, ColorOps};

#[cfg(test)]
mod sanity_test {
    #[test]
    fn it_works() {
        let result = 1 + 1;

        assert_eq!(result, 2);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("Hi!");
    }
}

#[cfg(test)]
mod rgb_test {    
    use super::{Rgb, Color};

    #[test]
    fn set_pt1() {
        let result = Rgb(0, 0, 0);

        assert_eq!(result.0, 0);
        assert_eq!(result.1, 0);
        assert_eq!(result.2, 0);
    }

    #[test]
    fn set_pt2() {
        let result = Rgb(255, 128, 255);

        assert_eq!(result.0, 255);
        assert_eq!(result.1, 128);
        assert_eq!(result.2, 255);
    }

    #[test]
    fn has_trait() {
        let result = Rgb(1, 6, 4);

        assert_eq!(result.r(), 1);
        assert_eq!(result.g(), 6);
        assert_eq!(result.b(), 4);
    }
}

#[cfg(test)]
mod hsv_test {    
    use super::{Hsv, Color};

    #[test]
    fn set_pt1() {
        let result = Hsv(0, 0.0, 0.0);

        assert_eq!(result.0, 0);
        assert!((result.1 - 0.0).abs() <= 1e-6);
        assert!((result.2 - 0.0).abs() <= 1e-6);
    }

    #[test]
    fn set_pt2() {
        let result = Hsv(45, 0.3333, 0.6666);

        assert_eq!(result.0, 45);
        assert!((result.1 - 0.3333).abs() <= 1e-6);
        assert!((result.2 -0.6666).abs() <= 1e-6);
    }

    #[test]
    fn has_trait() {
        let result = Hsv(121, 0.2, 0.4);

        assert_eq!(result.h(), 121);
        assert!((result.s() - 0.2).abs() <= 1e-6, "Expected s not equal to stored value: ({})", result.s());
        assert!((result.v() - 0.4).abs() <= 1e-6, "Expected v not equal to stored value: ({})", result.v());
    }
}

#[cfg(test)]
mod convert_test {    
    use super::{Rgb, Hsv, Color};

    #[test]
    fn rgb_to_hsv_pt1() {
        let a = Rgb(0, 0, 0);

        assert!((0..=0).contains(&a.h()), "Expected h not equal to stored value: ({})", a.h());
        assert!((a.s() - 0.0).abs() <= 1e-6, "Expected s not equal to stored value: ({})", a.s());
        assert!((a.v() - 0.0).abs() <= 1e-6, "Expected v not equal to stored value: ({})", a.s());
    }

    #[test]
    fn rgb_to_hsv_pt2() {
        let a = Rgb(123, 12, 230);

        assert!((270..=272).contains(&a.h()), "Expected h not equal to stored value: ({})", a.h());
        assert!((a.s() - 0.95).abs() <= 0.01, "Expected s not equal to stored value: ({})", a.s());
        assert!((a.v() - 0.90).abs() <= 0.01, "Expected v not equal to stored value: ({})", a.v());
    }

    #[test]
    fn rgb_to_hsv_pt3() {
        let a = Rgb(161, 163, 164);

        assert!((199..=201).contains(&a.h()), "Expected h not equal to stored value: ({})", a.h());
        assert!((a.s() - 0.02).abs() <= 0.01, "Expected s not equal to stored value: ({})", a.s());
        assert!((a.v() - 0.64).abs() <= 0.01, "Expected v not equal to stored value: ({})", a.v());
    }

    #[test]
    fn hsv_to_rgb_pt1() {
        let a = Hsv(90, 0.5, 0.75);

        assert!((142..=144).contains(&a.r()), "Expected r not equal to stored value: ({})", a.r());
        assert!((190..=192).contains(&a.g()), "Expected r not equal to stored value: ({})", a.g());
        assert!((94..=96).contains(&a.b()), "Expected r not equal to stored value: ({})", a.b());
    }

    #[test]
    fn hsv_to_rgb_pt2() {
        let a = Hsv(0, 0.0, 0.50);

        assert!((126..=128).contains(&a.r()), "Expected r not equal to stored value: ({})", a.r());
        assert!((126..=128).contains(&a.g()), "Expected r not equal to stored value: ({})", a.g());
        assert!((126..=128).contains(&a.b()), "Expected r not equal to stored value: ({})", a.b());
    }

    #[test]
    fn hsv_to_rgb_pt3() {
        let a = Hsv(26, 0.8, 0.43);

        assert!((108..=110).contains(&a.r()), "Expected r not equal to stored value: ({})", a.r());
        assert!((58..=60).contains(&a.g()), "Expected r not equal to stored value: ({})", a.g());
        assert!((20..=22).contains(&a.b()), "Expected r not equal to stored value: ({})", a.b());
    }
}

#[cfg(test)]
mod color_wheel_test {    
    use super::{Rgb, Hsv, Color, ColorOps};

    #[test]
    fn chroma_rgb_pt1() {
        let a = Rgb(0, 0, 0);
        let chroma_colors = a.chromatic(2);
        let color_res = vec![Rgb(0, 0, 0), Rgb(255, 255, 255)];

        for (each_color, each_color_ans) in chroma_colors.into_iter().zip(color_res) {
            assert!((each_color_ans.r().saturating_sub(2)..=each_color_ans.r().saturating_add(2)).contains(&each_color.r()), "Expected r not equal to stored value: ({})", each_color.r());
            assert!((each_color_ans.g().saturating_sub(2)..=each_color_ans.g().saturating_add(2)).contains(&each_color.g()), "Expected g not equal to stored value: ({})", each_color.g());
            assert!((each_color_ans.b().saturating_sub(2)..=each_color_ans.b().saturating_add(2)).contains(&each_color.b()), "Expected b not equal to stored value: ({})", each_color.b());
        }
    }

    #[test]
    fn chroma_rgb_pt2() {
        let a = Rgb(0, 0, 0);
        let chroma_colors = a.chromatic(3);
        let color_res = vec![Rgb(0, 0, 0), Rgb(127, 127, 127), Rgb(255, 255, 255)];

        for (each_color, each_color_ans) in chroma_colors.into_iter().zip(color_res) {
            assert!((each_color_ans.r().saturating_sub(2)..=each_color_ans.r().saturating_add(2)).contains(&each_color.r()), "Expected r not equal to stored value: ({})", each_color.r());
            assert!((each_color_ans.g().saturating_sub(2)..=each_color_ans.g().saturating_add(2)).contains(&each_color.g()), "Expected g not equal to stored value: ({})", each_color.g());
            assert!((each_color_ans.b().saturating_sub(2)..=each_color_ans.b().saturating_add(2)).contains(&each_color.b()), "Expected b not equal to stored value: ({})", each_color.b());
        }
    }

    #[test]
    fn chroma_rgb_pt3() {
        let a = Rgb(96, 127, 108);
        let chroma_colors = a.chromatic(10);
        let color_res = vec![Rgb(0, 0, 0), Rgb(21, 28, 23), Rgb(42, 56, 47), Rgb(63, 84, 71), Rgb(85, 112, 95), 
            Rgb(106, 140, 119), Rgb(127, 168, 143), Rgb(149, 196, 167), Rgb(170, 224, 191), Rgb(193, 255, 217)];

        for (each_color, each_color_ans) in chroma_colors.into_iter().zip(color_res) {
            assert!((each_color_ans.r().saturating_sub(2)..=each_color_ans.r().saturating_add(2)).contains(&each_color.r()), "Expected r not equal to stored value: ({})", each_color.r());
            assert!((each_color_ans.g().saturating_sub(2)..=each_color_ans.g().saturating_add(2)).contains(&each_color.g()), "Expected g not equal to stored value: ({})", each_color.g());
            assert!((each_color_ans.b().saturating_sub(2)..=each_color_ans.b().saturating_add(2)).contains(&each_color.b()), "Expected b not equal to stored value: ({})", each_color.b());
        }
    }

    #[test]
    fn chroma_hsv_pt1() {
        let a = Hsv(120, 0.12, 0.0);
        let chroma_colors = a.chromatic(2);
        let color_res = vec![Hsv(120, 0.12, 0.0), Hsv(120, 0.12, 1.0)];

        for (each_color, each_color_ans) in chroma_colors.into_iter().zip(color_res) {
            assert!((each_color_ans.h().saturating_sub(2)..=each_color_ans.h().saturating_add(2)).contains(&each_color.h()), "Expected h not equal to stored value: ({})", each_color.h());
            assert!((each_color_ans.s() - each_color.s()).abs() <= 0.01, "Expected s not equal to stored value: ({})", each_color.s());
            assert!((each_color_ans.v() - each_color.v()).abs() <= 0.01, "Expected v not equal to stored value: ({})", each_color.v());
        }
    }

    #[test]
    fn chroma_hsv_pt2() {
        let a = Hsv(164, 0.12, 0.12);
        let chroma_colors = a.chromatic(5);
        let color_res = vec![Hsv(164, 0.12, 0.0), Hsv(164, 0.12, 0.25), Hsv(164, 0.12, 0.5), Hsv(164, 0.12, 0.75), Hsv(164, 0.12, 1.0)];

        for (each_color, each_color_ans) in chroma_colors.into_iter().zip(color_res) {
            assert!((each_color_ans.h().saturating_sub(2)..=each_color_ans.h().saturating_add(2)).contains(&each_color.h()), "Expected h not equal to stored value: ({})", each_color.h());
            assert!((each_color_ans.s() - each_color.s()).abs() <= 0.01, "Expected s not equal to stored value: ({})", each_color.s());
            assert!((each_color_ans.v() - each_color.v()).abs() <= 0.01, "Expected v not equal to stored value: ({})", each_color.v());
        }
    }

    #[test]
    fn chroma_hsv_pt3() {
        let a = Hsv(66, 0.37, 0.5);
        let chroma_colors = a.chromatic(11);
        let color_res = vec![Hsv(66, 0.37, 0.0), Hsv(66, 0.37, 0.1), Hsv(66, 0.37, 0.2), Hsv(66, 0.37, 0.3), Hsv(66, 0.37, 0.4),
            Hsv(66, 0.37, 0.5), Hsv(66, 0.37, 0.6), Hsv(66, 0.37, 0.7), Hsv(66, 0.37, 0.8), Hsv(66, 0.37, 0.9),
            Hsv(66, 0.37, 1.0)];

        for (each_color, each_color_ans) in chroma_colors.into_iter().zip(color_res) {
            assert!((each_color_ans.h().saturating_sub(2)..=each_color_ans.h().saturating_add(2)).contains(&each_color.h()), "Expected h not equal to stored value: ({})", each_color.h());
            assert!((each_color_ans.s() - each_color.s()).abs() <= 0.01, "Expected s not equal to stored value: ({})", each_color.s());
            assert!((each_color_ans.v() - each_color.v()).abs() <= 0.01, "Expected v not equal to stored value: ({})", each_color.v());
        }
    }

    #[test]
    fn triad_rgb_pt1() {
        let a = Rgb(255, 255, 255);
        let chroma_colors = a.triad();
        let color_res = vec![Rgb(255, 255, 255), Rgb(255, 255, 255), Rgb(255, 255, 255)];

        for (each_color, each_color_ans) in Vec::from(chroma_colors).into_iter().zip(color_res) {
            assert!((each_color_ans.r().saturating_sub(2)..=each_color_ans.r().saturating_add(2)).contains(&each_color.r()), "Expected r not equal to stored value: ({})", each_color.r());
            assert!((each_color_ans.g().saturating_sub(2)..=each_color_ans.g().saturating_add(2)).contains(&each_color.g()), "Expected g not equal to stored value: ({})", each_color.g());
            assert!((each_color_ans.b().saturating_sub(2)..=each_color_ans.b().saturating_add(2)).contains(&each_color.b()), "Expected b not equal to stored value: ({})", each_color.b());
        }
    }

    #[test]
    fn triad_rgb_pt2() {
        let a = Rgb(47, 249, 125);
        let chroma_colors = a.triad();
        let color_res = vec![Rgb(47, 249, 125), Rgb(125, 47, 249), Rgb(249, 125, 47)];

        for (each_color, each_color_ans) in Vec::from(chroma_colors).into_iter().zip(color_res) {
            assert!((each_color_ans.r().saturating_sub(2)..=each_color_ans.r().saturating_add(2)).contains(&each_color.r()), "Expected r not equal to stored value: ({})", each_color.r());
            assert!((each_color_ans.g().saturating_sub(2)..=each_color_ans.g().saturating_add(2)).contains(&each_color.g()), "Expected g not equal to stored value: ({})", each_color.g());
            assert!((each_color_ans.b().saturating_sub(2)..=each_color_ans.b().saturating_add(2)).contains(&each_color.b()), "Expected b not equal to stored value: ({})", each_color.b());
        }
    }

    #[test]
    fn triad_rgb_pt3() {
        let a = Rgb(123, 231, 132);
        let chroma_colors = a.triad();
        let color_res = vec![Rgb(123, 231, 132), Rgb(132, 123, 231), Rgb(231, 132, 123)];

        for (each_color, each_color_ans) in Vec::from(chroma_colors).into_iter().zip(color_res) {
            assert!((each_color_ans.r().saturating_sub(2)..=each_color_ans.r().saturating_add(2)).contains(&each_color.r()), "Expected r not equal to stored value: ({})", each_color.r());
            assert!((each_color_ans.g().saturating_sub(2)..=each_color_ans.g().saturating_add(2)).contains(&each_color.g()), "Expected g not equal to stored value: ({})", each_color.g());
            assert!((each_color_ans.b().saturating_sub(2)..=each_color_ans.b().saturating_add(2)).contains(&each_color.b()), "Expected b not equal to stored value: ({})", each_color.b());
        }
    }

    #[test]
    fn complement_hsv_pt1() {
        let a = Hsv(180, 0.75, 0.8);
        let chroma_color = a.complement();
        let color_res = Hsv(0, 0.75, 0.8);

        assert!((color_res.h().saturating_sub(2)..=color_res.h().saturating_add(2)).contains(&chroma_color.h()), "Expected h not equal to stored value: ({})", chroma_color.h());
        assert!((chroma_color.s() - color_res.s()).abs() <= 0.01, "Expected s not equal to stored value: ({})", chroma_color.s());
        assert!((chroma_color.v() - color_res.v()).abs() <= 0.01, "Expected v not equal to stored value: ({})", chroma_color.v());
    }

    #[test]
    fn complement_hsv_pt2() {
        let a = Hsv(0, 0.0, 0.0);
        let chroma_color = a.complement();
        let color_res = Hsv(180, 0.0, 0.0);

        assert!((color_res.h().saturating_sub(2)..=color_res.h().saturating_add(2)).contains(&chroma_color.h()), "Expected h not equal to stored value: ({})", chroma_color.h());
        assert!((chroma_color.s() - color_res.s()).abs() <= 0.01, "Expected s not equal to stored value: ({})", chroma_color.s());
        assert!((chroma_color.v() - color_res.v()).abs() <= 0.01, "Expected v not equal to stored value: ({})", chroma_color.v());
    }

    #[test]
    fn complement_hsv_pt3() {
        let a = Hsv(280, 0.5, 0.5);
        let chroma_color = a.complement();
        let color_res = Hsv(100, 0.5, 0.5);

        assert!((color_res.h().saturating_sub(2)..=color_res.h().saturating_add(2)).contains(&chroma_color.h()), "Expected h not equal to stored value: ({})", chroma_color.h());
        assert!((chroma_color.s() - color_res.s()).abs() <= 0.01, "Expected s not equal to stored value: ({})", chroma_color.s());
        assert!((chroma_color.v() - color_res.v()).abs() <= 0.01, "Expected v not equal to stored value: ({})", chroma_color.v());
    }

    #[test]
    fn complement_rgb_pt1() {
        let a = Rgb(123, 231, 132);
        let chroma_colors = a.complement();
        let color_res = Rgb(232, 122, 222);

        assert!((color_res.r().saturating_sub(2)..=color_res.r().saturating_add(2)).contains(&chroma_colors.r()), "Expected r not equal to stored value: ({})", chroma_colors.r());
        assert!((color_res.g().saturating_sub(2)..=color_res.g().saturating_add(2)).contains(&chroma_colors.g()), "Expected g not equal to stored value: ({})", chroma_colors.g());
        assert!((color_res.b().saturating_sub(2)..=color_res.b().saturating_add(2)).contains(&chroma_colors.b()), "Expected b not equal to stored value: ({})", chroma_colors.b());
    }

    #[test]
    fn complement_rgb_pt2() {
        let a = Rgb(255, 255, 255);
        let chroma_colors = a.complement();
        let color_res = Rgb(255, 255, 255);

        assert!((color_res.r().saturating_sub(2)..=color_res.r().saturating_add(2)).contains(&chroma_colors.r()), "Expected r not equal to stored value: ({})", chroma_colors.r());
        assert!((color_res.g().saturating_sub(2)..=color_res.g().saturating_add(2)).contains(&chroma_colors.g()), "Expected g not equal to stored value: ({})", chroma_colors.g());
        assert!((color_res.b().saturating_sub(2)..=color_res.b().saturating_add(2)).contains(&chroma_colors.b()), "Expected b not equal to stored value: ({})", chroma_colors.b());
    }

    #[test]
    fn complement_rgb_pt3() {
        let a = Rgb(188, 90, 90);
        let chroma_colors = a.complement();
        let color_res = Rgb(90, 188, 188);

        assert!((color_res.r().saturating_sub(2)..=color_res.r().saturating_add(2)).contains(&chroma_colors.r()), "Expected r not equal to stored value: ({})", chroma_colors.r());
        assert!((color_res.g().saturating_sub(2)..=color_res.g().saturating_add(2)).contains(&chroma_colors.g()), "Expected g not equal to stored value: ({})", chroma_colors.g());
        assert!((color_res.b().saturating_sub(2)..=color_res.b().saturating_add(2)).contains(&chroma_colors.b()), "Expected b not equal to stored value: ({})", chroma_colors.b());
    }
}