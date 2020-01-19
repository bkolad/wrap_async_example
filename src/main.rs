#![deny(warnings)]

#[tokio::main]
async fn main() {
    let api = filters::all();

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

pub mod filters {
    use super::handlers;
    //use super::models::Todo;
    use warp::Filter;

    pub fn all() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        some_info1().or(some_info2())
    }

    pub fn some_info1() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("api" / "info1")
            .and(warp::get())
            .and_then(handlers::info1)
    }

    pub fn some_info2() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("api" / "info2")
            .and(warp::get())
            .and_then(handlers::info2)
    }
}

pub mod handlers {
    use std::convert::Infallible;
    use std::thread;
    use std::time::Duration;
    use tokio::task;
    use warp::http::StatusCode;

    pub async fn info1() -> Result<impl warp::Reply, Infallible> {
        task::spawn_blocking(|| {
            thread::sleep(Duration::from_secs(15));
            println!("spawn_blocking");
        });
        Ok(StatusCode::OK)
    }

    pub async fn info2() -> Result<impl warp::Reply, Infallible> {
        task::spawn_blocking(|| {
            thread::sleep(Duration::from_secs(6));
            println!("spawn_blocking");
        });

        task::spawn(async {
            tokio::time::delay_for(Duration::from_secs(5)).await;
            println!("spawn_non_blocking")
        });

        Ok(StatusCode::OK)
    }
}
