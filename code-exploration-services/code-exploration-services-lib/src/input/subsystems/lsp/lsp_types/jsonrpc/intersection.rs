#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Intersection<T0, T1>(pub T0, pub T1);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Intersection3<T0, T1, T2>(pub T0, pub T1, pub T2);
