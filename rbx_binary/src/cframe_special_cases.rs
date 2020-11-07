use rbx_dom_weak::types::{Matrix3, Vector3};

pub(crate) fn id_to_rotation(id: u8) -> Option<Matrix3> {
    match id {
        0x02 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x03 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x05 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x06 => Some(Matrix3::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x07 => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x09 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x0a => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x0c => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x0d => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x0e => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x10 => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x11 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        0x14 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x15 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x17 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x18 => Some(Matrix3::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x19 => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )),
        0x1b => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        )),
        0x1c => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        0x1e => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(0.0, -1.0, 0.0),
        )),
        0x1f => Some(Matrix3::new(
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        0x20 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        0x22 => Some(Matrix3::new(
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        0x23 => Some(Matrix3::new(
            Vector3::new(0.0, 0.0, -1.0),
            Vector3::new(0.0, -1.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        _ => None,
    }
}

#[allow(clippy::float_cmp)]
pub(crate) fn id_from_rotation(mat: Matrix3) -> Option<u8> {
    match mat {
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 1.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == 1.0 =>
        {
            Some(0x02u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == -1.0
            && z_x == 0.0
            && z_y == 1.0
            && z_z == 0.0 =>
        {
            Some(0x03u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == -1.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == -1.0 =>
        {
            Some(0x05u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == 1.0
            && z_x == 0.0
            && z_y == 1.0
            && z_z == 0.0 =>
        {
            Some(0x06u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 1.0
            && x_z == 0.0
            && y_x == 1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == -1.0 =>
        {
            Some(0x07u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == 1.0
            && y_x == 1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 1.0
            && z_z == 0.0 =>
        {
            Some(0x09u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == -1.0
            && x_z == 0.0
            && y_x == 1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == 1.0 =>
        {
            Some(0x0au8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == -1.0
            && y_x == 1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == -1.0
            && z_z == 0.0 =>
        {
            Some(0x0cu8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 1.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == 1.0
            && z_x == 1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x0du8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == -1.0
            && y_x == 0.0
            && y_y == 1.0
            && y_z == 0.0
            && z_x == 1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x0eu8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == -1.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == -1.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == 1.0 =>
        {
            Some(0x10u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == 1.0
            && y_x == 0.0
            && y_y == -1.0
            && y_z == 0.0
            && z_x == 1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x11u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == -1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 1.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == -1.0 =>
        {
            Some(0x14u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == -1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == 1.0
            && z_x == 0.0
            && z_y == 1.0
            && z_z == 0.0 =>
        {
            Some(0x15u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == -1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == -1.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == 1.0 =>
        {
            Some(0x17u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == -1.0
            && x_y == 0.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == -1.0
            && z_x == 0.0
            && z_y == -1.0
            && z_z == 0.0 =>
        {
            Some(0x18u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 1.0
            && x_z == 0.0
            && y_x == -1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == 1.0 =>
        {
            Some(0x19u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == -1.0
            && y_x == -1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 1.0
            && z_z == 0.0 =>
        {
            Some(0x1bu8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == -1.0
            && x_z == 0.0
            && y_x == -1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == 0.0
            && z_z == -1.0 =>
        {
            Some(0x1cu8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == 1.0
            && y_x == -1.0
            && y_y == 0.0
            && y_z == 0.0
            && z_x == 0.0
            && z_y == -1.0
            && z_z == 0.0 =>
        {
            Some(0x1eu8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 1.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == -1.0
            && z_x == -1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x1fu8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == 1.0
            && y_x == 0.0
            && y_y == 1.0
            && y_z == 0.0
            && z_x == -1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x20u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == -1.0
            && x_z == 0.0
            && y_x == 0.0
            && y_y == 0.0
            && y_z == 1.0
            && z_x == -1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x22u8)
        }
        Matrix3 {
            x:
                Vector3 {
                    x: x_x,
                    y: x_y,
                    z: x_z,
                },
            y:
                Vector3 {
                    x: y_x,
                    y: y_y,
                    z: y_z,
                },
            z:
                Vector3 {
                    x: z_x,
                    y: z_y,
                    z: z_z,
                },
        } if x_x == 0.0
            && x_y == 0.0
            && x_z == -1.0
            && y_x == 0.0
            && y_y == -1.0
            && y_z == 0.0
            && z_x == -1.0
            && z_y == 0.0
            && z_z == 0.0 =>
        {
            Some(0x23u8)
        }
        _ => None,
    }
}
