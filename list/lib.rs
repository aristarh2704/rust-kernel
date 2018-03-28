#![no_std]
#![feature(const_fn)]
#[macro_use]
extern crate console;
#[repr(packed)]
pub struct List<'a,T:'a>{
    pub element: T,
    pub next: Option<&'a mut List<'a,T>>
}
pub struct FirstNode<'a,T:'a>{
    next: Option<&'a mut List<'a,T>>,
}
pub struct ListIter<'b, 'a:'b,T:'a>{
    next: Option<&'b mut List<'a,T>>,
}
impl<'a,T> FirstNode<'a,T>{
    pub const fn new()->Self{
        Self{
            next: None
        }
    }
    pub fn insert_begin(&mut self,addr:&'static mut List<'a,T>){
        let mut temp=None;
        core::mem::swap(&mut self.next,&mut temp);
        addr.next=temp;
        self.next=Some(addr);
    }
    pub fn iter<'b>(&'b mut self)->ListIter<'b,'a,T>{
        ListIter{
            next: match self.next{
                Some(ref mut n)=>Some(&mut **n),
                None=>None
            }
        }
    }
}

impl<'a,'b,T> Iterator for ListIter<'b,'a,T>{
    type Item=&'b mut T;
    fn next(&mut self)->Option<Self::Item>{
        let mut temp=None;
        core::mem::swap(&mut self.next,&mut temp);  // Now temp is Option(Pointer to current List), self.next is None
        match temp{
            None=>return None,
            Some(n)=>{
                // n is pointer to current list, temp is terminated
                // self.next must be copy n.next, it borrow n, but we must return Some(&mut n.element)
                // Hm...
                self.next=match n.next{
                    None=>None,
                    Some(ref mut m)=>Some(&mut **m)
                };
                return Some(&mut n.element);
            }
        }
    }
 }