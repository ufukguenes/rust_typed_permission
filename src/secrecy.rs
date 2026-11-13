pub struct S0 {}
impl Level0 for S0 {}
impl Level1 for S0 {}
impl Level2 for S0 {}
impl Level3 for S0 {}

pub struct S1 {}
impl Level1 for S1 {}
impl Level2 for S1 {}
impl Level3 for S1 {}

pub struct S2 {}
impl Level2 for S2 {}
impl Level3 for S2 {}

pub struct S3 {}
impl Level3 for S3 {}

pub trait Level0 {}
pub trait Level1 {}
pub trait Level2 {}
pub trait Level3 {}
