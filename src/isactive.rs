pub trait IsActive{
    fn is_active(&self) ->bool;
    fn kill(&mut self);
}
