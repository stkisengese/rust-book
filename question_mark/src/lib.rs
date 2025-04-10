pub struct One {
    pub first_layer: Option<Two>,
}
pub struct Two {
    // expected public fields
    pub second_layer: Option<Three>,
}
pub struct Three {
    pub third_layer: Option<Four>,
}
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer?.second_layer?.third_layer?.fourth_layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000)
                    })
                })
            })
        };

        let result = a.get_fourth_layer();
        assert_eq!(result, Some(1000));
    }
}
