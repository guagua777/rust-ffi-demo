extern crate libc;

use libc::c_char;
use std::collections::HashMap;
use std::ffi::CStr;

pub struct ZipCodeDatabase {
    population: HashMap<String, u32>,
}

// One extern function is created for each function of the object
// 该struct的每个函数都创建了一个外部函数
impl ZipCodeDatabase {
    fn new() -> ZipCodeDatabase {
        ZipCodeDatabase {
            population: HashMap::new(),
        }
    }

    fn populate(&mut self) {
        for i in 0..100_000 {
            let zip = format!("{:05}", i);
            self.population.insert(zip, i);
        }
    }

    fn population_of(&self, zip: &str) -> u32 {
        self.population.get(zip).cloned().unwrap_or(0)
    }
}

// 返回原始指针
#[unsafe(no_mangle)]
pub extern "C" fn zip_code_database_new() -> *mut ZipCodeDatabase {
    // 想想为什么要装箱
    Box::into_raw(Box::new(ZipCodeDatabase::new()))
}

// 参数为原始指针
#[unsafe(no_mangle)]
pub extern "C" fn zip_code_database_free(ptr: *mut ZipCodeDatabase) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

// 从原始指针创建引用
// To create a reference from a raw pointer, you can use the terse syntax &*, 
// which indicates that the pointer should be dereferenced and then re-referenced.
// 创建可变引用类似，但使用 &mut * 

// 参数为原始指针
#[unsafe(no_mangle)]
pub extern "C" fn zip_code_database_populate(ptr: *mut ZipCodeDatabase) {
    let database = unsafe {
        assert!(!ptr.is_null());
        // 1. *ptr 解引用原始指针，得到 ZipCodeDatabase
        // 2. &mut 获取 ZipCodeDatabase 的可变引用
        &mut *ptr
    };
    database.populate();
}


// 参数为const
#[unsafe(no_mangle)]
pub extern "C" fn zip_code_database_population_of(
    ptr: *const ZipCodeDatabase,
    zip: *const c_char,
) -> u32 {
    let database = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let zip = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip)
    };
    let zip_str = zip.to_str().unwrap();
    database.population_of(zip_str)
}