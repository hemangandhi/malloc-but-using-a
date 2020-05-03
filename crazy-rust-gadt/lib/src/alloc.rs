use std::marker::PhantomData;

pub static CELL_DATA: [PhantomData<T: Freeness>; 100] = [IsFree; 100];
