1. HttpServer 是负责服务的HTTP 请求


  （1） 实例化

        HttpServer::new(factory:F)

   (2) 绑定服务IP地址和端口号，即运行的地址

        HttpServer::new(factory:F).bind("127.0.0.1:8080")

        运行在本地的8080 端口号上
    
   (3) 运行

        HttpServer::new(factory:F).bind("127.0.0.1:8080")?.run()
    (4) 因为是异步的，使用await

        HttpServer::new(factory:F).bind("127.0.0.1:8080")?.run().await

2. 多线程

   HttpServer自动启动一些HTTP Worker，默认情况下，此数目等于系统中逻辑CPU的数目。该HttpServer::workers()方法可以覆盖此数字 。


        use actix_web::{web, App, HttpResponse, HttpServer};

        #[actix_web::main]
        async fn main() {
            HttpServer::new(|| {
                App::new().route("/", web::get().to(|| HttpResponse::Ok()))
            })
            .workers(4); // <- Start 4 workers
        }
