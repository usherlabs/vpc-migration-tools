# **VPC Migration Tools**

VPC Migration Tools is a Rust CLI application designed to automate software configuration and image creation tasks. The primary aim is to facilitate smoother migrations when necessary. It is currently optimized for specific use cases, such as Ubuntu 22.04, but we plan to broaden its compatibility over time. The tool has been crafted to align with the requirements of IBM Cloud VPC migration based on this **[IBM documentation](https://cloud.ibm.com/docs/vpc?topic=vpc-create-linux-custom-image)**.

## **Features**

- Automated compatibility check to determine system readiness.
- Automated image creation and space optimization to streamline migration
- 
## **Installation**

Before you start, make sure you have Rust and Cargo installed on your system. If you haven't, you can install them from the **[official site](https://www.rust-lang.org/tools/install)**.

To install VPC Migration Tools:

1. Clone the repository: **`git clone https://github.com/usherlabs/vpc-migration-tools.git`**
2. Navigate to the project directory: **`cd vpc-migration-tools`**
3. Build the project: **`cargo build --release`**

The executable will be in the **`./target/release/`** directory.

You might want to use `scp` to transfer the executable to the server via SSH.

```shell
scp ./target/release/vpc-migration-tools <username>@<server-ip>:<path>
```

## **Usage**

VPC Migration Tools provides two main commands. For more detailed information on each command, use **`./vpc-migration-tools help <command>`**:

- Check software configuration: **`./vpc-migration-tools check-requirements`**
- Create a custom software image: **`./vpc-migration-tools create-image`**

Please note, it may be necessary to use sudo to execute the commands.

## **Migration Steps**

Before proceeding with the migration, it is essential to check whether your system is compatible with the VPC Migration Tools. To do this, run the **`check-requirements`** command. The logs produced will indicate whether your system meets the necessary criteria.

If your system is not compatible, you should not proceed with the automated solution but instead address the issue manually. Once the requirements are met, you can then move on to the image creation step. It may be necessary to reboot your system after resolving any compatibility issues.

## **Contributing**

We welcome contributions to VPC Migration Tools. To contribute:

1. Fork the project.
2. Create your feature branch (**`git checkout -b feature/AmazingFeature`**).
3. Commit your changes (**`git commit -m 'Add some AmazingFeature'`**).
4. Push to the branch (**`git push origin feature/AmazingFeature`**).
5. Open a Pull Request.

## **Testing**

To run the tests for VPC Migration Tools, use the following command:

```
cargo test
```

## **License**

TODO
