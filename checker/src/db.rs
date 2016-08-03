extern crate mysql;

use ::CheckerErr;

pub fn get_addrs(pool: &mysql::Pool) -> Vec<(u64, String)>
{
    pool.prep_exec("SELECT * FROM `checker`.`services`", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {

            let (id, ip, _, _, _) = mysql::from_row::<(u64,String,String,String,i64)>(row);
            (id, ip)
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
