use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Write};
use actix_web::Resource;
use cached::{proc_macro::{ io_cached, cached }, AsyncRedisCache};
extern crate alloc;

// _noexpand_types: dict[osuTypes, Callable[..., bytes]] = {
//     # base
//     osuTypes.i8: struct.Struct("<b").pack,
//     osuTypes.u8: struct.Struct("<B").pack,
//     osuTypes.i16: struct.Struct("<h").pack,
//     osuTypes.u16: struct.Struct("<H").pack,
//     osuTypes.i32: struct.Struct("<i").pack,
//     osuTypes.u32: struct.Struct("<I").pack,
//     # osuTypes.f16: struct.Struct('<e').pack, # futureproofing
//     osuTypes.f32: struct.Struct("<f").pack,
//     osuTypes.i64: struct.Struct("<q").pack,
//     osuTypes.u64: struct.Struct("<Q").pack,
//     osuTypes.f64: struct.Struct("<d").pack,
//     # more complex
//     osuTypes.string: write_string,
//     osuTypes.i32_list: write_i32_list,
//     osuTypes.scoreframe: write_scoreframe,
//     # TODO: write replayframe & bundle
// }

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<i8, [u8; 1]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_i8", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_i8(value: i8) -> Result<[u8; 1], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<u8, [u8; 1]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_u8", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_u8(value: u8) -> Result<[u8; 1], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<i16, [u8; 2]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_i16", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_i16(value: i16) -> Result<[u8; 2], crate::RedisError> {
     Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<u16, [u8; 2]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_u16", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_u16(value: u16) -> Result<[u8; 2], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<i32, [u8; 4]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_i32", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_i32(value: i32) -> Result<[u8; 4], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<u32, [u8; 4]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_u32", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_u32(value: u32) -> Result<[u8; 4], crate::RedisError> {
    Ok(value.to_le_bytes())
}

pub async fn write_f16(value: half::f16) -> Result<[u8; 2], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<f32, [u8; 4]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_f32", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_f32(value: f32) -> Result<[u8; 4], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<i64, [u8; 8]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_i64", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_i64(value: i64) -> Result<[u8; 8], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<u64, [u8; 8]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_u64", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_u64(value: u64) -> Result<[u8; 8], crate::RedisError> {
    Ok(value.to_le_bytes())
}

#[io_cached(
    map_error = r##"|e| crate::RedisError::RedisError(format!("{:?}", e))"##,
    type = "cached::AsyncRedisCache<f64, [u8; 8]>",
    create = r##" {
        AsyncRedisCache::new("bancho-rs_cache_write_f64", 1)
            .set_refresh(true)
            .build()
            .await
            .expect("error building example redis cache")
    } "##
)]
pub async fn write_f64(value: f64) -> Result<[u8; 8], crate::RedisError> {
    Ok(value.to_le_bytes())
}

pub async fn write_uleb128(mut value: u8) -> Vec<u8> {
    let mut result = Vec::new();

    loop {
        let mut byte = value & 0x7f;
        value >>= 7;

        if value == 0 {
            byte |= 0x80;
        }

        result.push(byte);

        if value == 0 {
            break;
        }
    }
    result
}

pub async fn write_string(value: &str) -> Vec<u8> {
    //! god forgive us
    if value == "" {
        return b"\x00".to_vec();
    }
    let mut r = Vec::new();
    r.extend(b"\x00".to_vec());
    r.extend(write_uleb128(value.as_bytes().len().try_into().unwrap()).await);
    r.extend(value.as_bytes().to_vec());
    r
}

pub async fn write_i32_list(value: Vec<i32>) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend(write_i16(value.len().try_into().unwrap()).await.unwrap());
    for i in value {
        r.extend(write_i32(i).await.unwrap());
    }
    r
}

pub async fn write_message<T, U, V, W>(sender: T, msg: U, recipient: V, sender_id: W) -> Vec<u8>
where
    T: Into<&'static str>,
    U: Into<&'static str>,
    V: Into<&'static str>,
    W: Into<u32>,
{
    let mut r = Vec::new();
    r.extend(write_string(sender.into()).await);
    r.extend(write_string(msg.into()).await);
    r.extend(write_string(recipient.into()).await);
    r.extend(write_u32(sender_id.into()).await.unwrap());
    r
}

pub async fn write_channel<T, U, V>(name: T, topic: U, count: V) -> Vec<u8>
where
    T: Into<&'static str>,
    U: Into<&'static str>,
    V: Into<u16>,
{
    let mut r = Vec::new();
    r.extend(write_string(name.into()).await);
    r.extend(write_string(topic.into()).await);
    r.extend(write_u16(count.into()).await.unwrap());
    r
}

pub async fn write<T>(value: T) -> Vec<u8> {
    if std::any::type_name::<T>() == "i8" {
        return write_i8(unsafe { std::mem::transmute_copy::<T, i8>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "u8" {
        return write_u8(unsafe { std::mem::transmute_copy::<T, u8>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "i16" {
        return write_i16(unsafe { std::mem::transmute_copy::<T, i16>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "u16" {
        return write_u16(unsafe { std::mem::transmute_copy::<T, u16>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "i32" {
        return write_i32(unsafe { std::mem::transmute_copy::<T, i32>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "u32" {
        return write_u32(unsafe { std::mem::transmute_copy::<T, u32>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "half::binary16::f16" {
        return write_f16(unsafe { std::mem::transmute_copy::<T, half::f16>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "f32" {
        return write_f32(unsafe { std::mem::transmute_copy::<T, f32>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "i64" {
        return write_i64(unsafe { std::mem::transmute_copy::<T, i64>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "u64" {
        return write_u64(unsafe { std::mem::transmute_copy::<T, u64>(&value) }).await.unwrap().to_vec();
    }
    if std::any::type_name::<T>() == "f64" {
        return write_f64(unsafe { std::mem::transmute_copy::<T, f64>(&value) }).await.unwrap().to_vec();
    }
    // if std::any::type_name::<T>() == "alloc::string::String" {
    //     return write_string(unsafe { std::mem::transmute_copy::<T, alloc::string::String>(&value).as_str() });
    // }
    if std::any::type_name::<T>() == "&str" {
        return write_string(unsafe { std::mem::transmute_copy::<T, &str>(&value) }).await;
    }
    b"\x00".to_vec()
}

#[actix_web::test]
async fn test_write() {
    println!("i8: {:x?}", write(1 as i8).await);
    println!("u8: {:x?}", write(1 as u8).await);
    println!("i16: {:x?}", write(1 as i16).await);
    println!("u16: {:x?}", write(1 as u16).await);
    println!("i32: {:x?}", write(1 as i32).await);
    println!("u32: {:x?}", write(1 as u32).await);
    //println!("{}", std::any::type_name::<half::f16>());
    println!("f16: {:x?}", write(half::f16::from_f32(1 as f32)).await);
    println!("f32: {:x?}", write(1 as f32).await);
    println!("i64: {:x?}", write(1 as i64).await);
    println!("u64: {:x?}", write(1 as u64).await);
    println!("f64: {:x?}", write(1 as f64).await);
    println!("&str: {:x?}", write("test").await);
    //println!("{}", std::any::type_name::<String>());
    //println!("alloc::string::String: {:x?}", write("test".to_string()));
}
