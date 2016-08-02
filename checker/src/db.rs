extern crate mysql;

struct Service {
    id: u64,
    ip: String,
    name: String,
    token: String,
    points: u64
}

pub fn get_addrs(pool: &mysql::Pool) -> Vec<(u64, String)>
{
    let services: Vec<Service> = pool.prep_exec("SELECT * FROM `checker`.`services`", ())
    .map(|result| {
        result.map(|x| x.unwrap()).map(|row| {

            let (id, ip, name, token, points) = mysql::from_row(row);
            Service{id: id, ip: ip, name: name, token: token, points: points}
        }).collect()
    }).unwrap();
    services.iter().map(|s| (s.id, s.ip.clone())).collect()
}
