extern crate mysql;

use ::CheckerErr;

pub struct Service
{
    pub id: u64,
    pub ip: String,
    pub name: String
}

pub fn get_addrs(pool: &mysql::Pool) -> Vec<Service>
{
    pool.prep_exec("SELECT * FROM `checker`.`services`", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {

            let (id, ip, name, _, _) = mysql::from_row::<(u64,String,String,String,i64)>(row);
            Service{id: id, ip: ip, name: name}
        }).collect()
    }).unwrap()
}

pub fn give_penalty(pool: &mysql::Pool, service_id: u64, peny: u32, reason: &CheckerErr)
{
    pool.prep_exec("UPDATE `checker`.`services` SET `points`=`points`-:peny WHERE `id` = :service",
                   (mysql::Value::from(peny), mysql::Value::from(service_id))).unwrap();
    pool.prep_exec("INSERT INTO `checker`.`log`(`id`, `service_id`, `event`, `points`) VALUES (0,:service,:event,:peny)",
                   (
                       mysql::Value::from(service_id),
                       mysql::Value::from(format!("{:?}",reason)),
                       mysql::Value::from(-(peny as i64))
                   )
                  ).unwrap();
}

pub fn reg_flag(pool: &mysql::Pool, flag: String, owner_id: u64)
{
    pool.prep_exec("INSERT INTO `checker`.`flags`(`id`, `flag`, `owner_id`, `used`) VALUES (0,:flag,:owner_id,0)",
                   (
                       mysql::Value::from(flag),
                       mysql::Value::from(owner_id)
                   )
                  ).unwrap();
}

pub fn spoil_flags(pool: &mysql::Pool)
{
    pool.prep_exec("UPDATE `checker`.`flags` SET `used`='-1'", ()).unwrap();
}
