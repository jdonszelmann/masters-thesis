pub struct RequestType0<R, E, RO> {
    pub method: &'static str,

    pub number_of_params: i32,

    pub __: Option<(R, E, RO)>,
}

pub struct RequestType<P, R, E, RO> {
    pub method: &'static str,

    pub number_of_params: i32,

    pub __: Option<(P, R, E, RO)>,
}
