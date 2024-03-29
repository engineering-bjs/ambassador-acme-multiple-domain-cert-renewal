use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;


fn main() {
    update_acme_service().expect("TODO: panic message");
}

fn update_acme_service() -> std::io::Result<()> {
    // let namespaces = ["production","staging","airflow","development","integrations"];
    let namespaces = ["staging"];

    return Ok(for namespace in namespaces {
        println!("namespace :- {}", namespace);

        // list of all pending ingress
        let result = Command::new("sh")
            .arg("-c")
            .arg(
                "kubectl get ingress -n ".to_owned()
                    + &namespace
                    + " | awk '{print $1}' | grep -e 'acme'",
            )
            .output()
            .expect("failed to get ingress");

        // continue if ingress are none in the namespaces
        if !result.status.success() {
            continue;
        }

        return Ok(for i in String::from_utf8(result.stdout).unwrap().split("\n") {
            // fetch detail of ingress pending for acme
            println!("pending ingress name :- {}", i);
            let mut result = Command::new("sh")
                .arg("-c")
                .arg(
                    "kubectl describe ingress ".to_owned()
                        + &i.to_string()
                        + " -n "
                        + &namespace
                        + " | awk '{print $0}' | grep -e 'cert-manager'",
                )
                .output()
                .expect("failed to get ingress details");
            assert!(result.status.success());
            let ingres_description;
            ingres_description = String::from_utf8(result.stdout).unwrap();
            // fetch domain name of ingress to find the service mapping
            result = Command::new("sh")
                .arg("-c")
                .arg(
                    " kubectl get ingress ".to_owned()+ " -n "
                        + &namespace
                        + " | grep "+&i.to_string()+" | awk '{print $3}'",
                )
                .output()
                .expect("failed to get ingress details");

            assert!(result.status.success());
            let ingress_domain;

             ingress_domain = String::from_utf8(result.stdout).unwrap();
             println!("ingress domain name :- {}", ingress_domain);

            // fetch service and fine associated ingress
            let v: Vec<&str> = ingress_domain.split(".").collect();

            let svc_name;

            svc_name = "acme-challenge-".to_owned() + v[0].trim() + "-mapping-service";

            println!("ingress acme service name :- {}", svc_name);

            // create new file name
            let file_name;
            file_name = svc_name.trim().to_owned() + ".yaml";

            let ingres_description = ingres_description.replace("Labels:", "");

            println!("ingres_description :- {}", &ingres_description);

            let mut v: Vec<&str> = ingres_description.split("http-domain").collect();
            v = v[1].split("http-token=").collect();
            let mut v1: Vec<&str> = v[0].split("\n").collect();
            let v2: Vec<&str> = v[1].split("\n").collect();

            // get http_domain value from sub string
            v1 = v1[0].split("=").collect();
            let http_domain = String::from(v1[1]);
            // get the http domain and token string
            println!("http domain {}", http_domain);

            // get http_token value from sub string
            let http_token = String::from(v2[0]);

            println!("http token {}", http_token);

            let acme_file;

            acme_file = "apiVersion: v1\n".to_owned()
                + "kind: Service\n"
                + "metadata:\n"
                + "  name: "
                + svc_name.trim()
                + "\n"
                + "  namespace: "
                + &namespace
                + "\n"
                + "spec:\n"
                + "  ports:\n"
                + "  - port: 80 \n"
                + "    protocol: TCP\n"
                + "    targetPort: 8089\n"
                + "  selector:\n"
                + "    acme.cert-manager.io/http-domain : '"
                + &http_domain
                + "'\n"
                + "    acme.cert-manager.io/http-token : '"
                + &http_token
                + "'\n"
                + "    acme.cert-manager.io/http01-solver : 'true' \n"
                + "  type: ClusterIP";
            println!("{}", acme_file);
            // write yaml into the file
            let mut file = File::create(&file_name)?;
            file.write(acme_file.as_bytes())?;

            println!("{}", "kubectl apply -f ".to_owned() + &file_name);
            // update the acme service by kubectl command
            let result = Command::new("sh")
                .arg("-c")
                .arg("kubectl apply -f ".to_owned() + &file_name)
                .output()
                .expect("failed to get ingress");
            assert!(result.status.success());

            // delete yaml file
            fs::remove_file(&file_name)?;
        });
    });
}
