pub struct NotificationType0<RO> {
    pub method: &'static str,

    pub number_of_params: i32,

    pub __: Option<RO>,
}

pub struct NotificationType<P, RO> {
    pub method: &'static str,

    pub number_of_params: i32,

    pub __: Option<(P, RO)>,
}
