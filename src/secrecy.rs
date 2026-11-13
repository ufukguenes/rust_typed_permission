pub struct S0 {}
impl Level0 for S0 {}
impl Level1 for S0 {}
impl Level2 for S0 {}
impl Level3 for S0 {}
impl PrivateLevel0 for S0 {}
impl PrivateLevel1 for S0 {}
impl PrivateLevel2 for S0 {}
impl PrivateLevel3 for S0 {}

pub struct S1 {}
impl Level1 for S1 {}
impl Level2 for S1 {}
impl Level3 for S1 {}
impl PrivateLevel1 for S1 {}
impl PrivateLevel2 for S1 {}
impl PrivateLevel3 for S1 {}

pub struct S2 {}
impl Level2 for S2 {}
impl Level3 for S2 {}
impl PrivateLevel2 for S2 {}
impl PrivateLevel3 for S2 {}

pub struct S3 {}
impl Level3 for S3 {}
impl PrivateLevel3 for S3 {}

trait PrivateLevel0 {}
trait PrivateLevel1 {}
trait PrivateLevel2 {}
trait PrivateLevel3 {}

pub trait Level0: PrivateLevel0 {}
pub trait Level1: PrivateLevel1 {}
pub trait Level2: PrivateLevel2 {}
pub trait Level3: PrivateLevel3 {}
