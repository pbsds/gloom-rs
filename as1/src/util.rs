use std::ffi::CString;

pub unsafe fn get_gl_string(name: gl::types::GLenum) -> String {
    std::ffi::CStr::from_ptr(gl::GetString(name) as *mut i8)
        .to_string_lossy()
        .to_string()
}

// Debug callback to panic upon enountering any OpenGL error
pub extern "system" fn debug_callback(
    source: u32,
    e_type: u32,
    id: u32,
    severity: u32,
    _length: i32,
    msg: *const i8,
    _data: *mut std::ffi::c_void,
) {
    if e_type != gl::DEBUG_TYPE_ERROR {
        return;
    }
    if severity == gl::DEBUG_SEVERITY_HIGH
        || severity == gl::DEBUG_SEVERITY_MEDIUM
        || severity == gl::DEBUG_SEVERITY_LOW
    {
        let severity_string = match severity {
            gl::DEBUG_SEVERITY_HIGH => "high",
            gl::DEBUG_SEVERITY_MEDIUM => "medium",
            gl::DEBUG_SEVERITY_LOW => "low",
            _ => "unknown",
        };
        unsafe {
            let string = CString::from_raw(msg as *mut i8);
            let error_message = String::from_utf8_lossy(string.as_bytes()).to_string();
            panic!(
                "{}: Error of severity {} raised from {}: {}\n",
                id, severity_string, source, error_message
            );
        }
    }
}

pub enum Direction {
    X,
    Y,
    Z,
}

pub unsafe fn get_rotation_matrix(theta: f32, dir: Direction) -> glm::Mat3 {
    let cos = theta.cos();
    let sin = theta.sin();
    match dir {
        Direction::X => get_rotation_matrix_x(sin, cos),
        Direction::Y => get_rotation_matrix_y(sin, cos),
        Direction::Z => get_rotation_matrix_z(sin, cos),
    }
}

unsafe fn get_rotation_matrix_x(sin: f32, cos: f32) -> glm::Mat3 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    glm::mat3(
        1.0, 0.0, 0.0,
        0.0, cos, -sin,
        0.0, sin,  cos,
    )
}

unsafe fn get_rotation_matrix_y(sin: f32, cos: f32) -> glm::Mat3 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    glm::mat3(
        cos, 0.0, sin,
        0.0, 1.0, 0.0,
        -sin, 0.0,  cos,
    )
}

unsafe fn get_rotation_matrix_z(sin: f32, cos: f32) -> glm::Mat3 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    glm::mat3(
        cos, -sin, 0.0,
        sin, cos,  0.0,
        0.0, 0.0,  1.0,
    )
}
