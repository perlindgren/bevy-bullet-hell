// utilities for game dev
//

#[derive(Debug, Copy, Clone)]
pub struct EnvPoint {
    pub delta_time: f32,
    pub value: f32,
}

#[derive(Debug, Clone, Default)]
pub struct Envelope {
    pub start_value: f32,
    pub points: Vec<EnvPoint>,
}

impl Envelope {
    pub fn get(&self, at_time: f32) -> f32 {
        let mut curr_time = 0.0;
        let mut curr_value = self.start_value;
        for v in self.points.iter() {
            println!("v {:?}", v);
            let next_time = curr_time + v.delta_time;
            if at_time < next_time {
                let time_dist = at_time - curr_time;
                let time_ratio = time_dist / v.delta_time;
                let value_dist = v.value - curr_value;
                curr_value += time_ratio * value_dist;
                break;
            } else {
                curr_time += v.delta_time;
                curr_value = v.value;
            }
        }
        curr_value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_envelope() {
        fn t(time: f32, envelope: &Envelope, expected: f32) {
            let value = envelope.get(time);
            assert_eq!(value, expected)
        }
        let envelope = Envelope {
            start_value: 4.0,
            points: vec![
                EnvPoint {
                    delta_time: 2.0,
                    value: 5.0,
                },
                EnvPoint {
                    delta_time: 4.0,
                    value: 1.0,
                },
            ],
        };

        t(0.0, &envelope, 4.0);
        t(0.5, &envelope, 4.25);
        t(1.0, &envelope, 4.5);
        t(2.0, &envelope, 5.0);
        t(4.0, &envelope, 3.0);
        t(6.0, &envelope, 1.0);
        t(7.0, &envelope, 1.0);

        let envelope = Envelope {
            start_value: 0.0,
            points: vec![EnvPoint {
                delta_time: 1.0,
                value: 1.0,
            }],
        };

        t(0.0, &envelope, 0.0);
        t(0.5, &envelope, 0.5);
        t(1.0, &envelope, 1.0);
        t(1.5, &envelope, 1.0);

        let envelope = Envelope {
            start_value: 1.0,
            points: vec![],
        };
        t(0.0, &envelope, 1.0);
        t(0.5, &envelope, 1.0);
        t(1.0, &envelope, 1.0);
        t(1.5, &envelope, 1.0);

        let envelope = Envelope {
            start_value: 1.0,
            points: vec![
                EnvPoint {
                    delta_time: 1.0,
                    value: 2.0,
                },
                EnvPoint {
                    delta_time: 1.0,
                    value: 0.0,
                },
            ],
        };

        t(0.0, &envelope, 1.0);
        t(0.5, &envelope, 1.5);
        t(0.75, &envelope, 1.75);

        t(1.0, &envelope, 2.0);
        t(1.5, &envelope, 1.0);

        t(2.0, &envelope, 0.0);
        t(2.5, &envelope, 0.0);
    }
}
