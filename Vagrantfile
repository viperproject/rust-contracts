# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure("2") do |config|
  # The most common configuration options are documented and commented below.
  # For a complete reference, please see the online documentation at
  # https://docs.vagrantup.com.

  # Every Vagrant development environment requires a box. You can search for
  # boxes at https://vagrantcloud.com/search.
  config.vm.box = "ubuntu/bionic64"

  # Disable automatic box update checking. If you disable this, then
  # boxes will only be checked for updates when the user runs
  # `vagrant box outdated`. This is not recommended.
  config.vm.box_check_update = true

  # Create a forwarded port mapping which allows access to a specific port
  # within the machine from a port on the host machine. In the example below,
  # accessing "localhost:8080" will access port 80 on the guest machine.
  # NOTE: This will enable public access to the opened port
  config.vm.network "forwarded_port", guest: 8080, host: 32134

  # Create a forwarded port mapping which allows access to a specific port
  # within the machine from a port on the host machine and only allow access
  # via 127.0.0.1 to disable public access
  # config.vm.network "forwarded_port", guest: 80, host: 8080, host_ip: "127.0.0.1"

  # Create a private network, which allows host-only access to the machine
  # using a specific IP.
  # config.vm.network "private_network", ip: "192.168.33.10"

  # Create a public network, which generally matched to bridged network.
  # Bridged networks make the machine appear as another physical device on
  # your network.
  # config.vm.network "public_network"

  # Share an additional folder to the guest VM. The first argument is
  # the path on the host to the actual folder. The second argument is
  # the path on the guest to mount the folder. And the optional third
  # argument is a set of non-required options.
  # config.vm.synced_folder "../data", "/vagrant_data"

  # Provider-specific configuration so you can fine-tune various
  # backing providers for Vagrant. These expose provider-specific options.
  # Example for VirtualBox:
  #
  config.vm.provider "virtualbox" do |vb|
    # Customize the amount of memory on the VM:
    vb.memory = "2048"
    vb.cpus = "1"
  end
  #
  # View the documentation for the provider you are using for more
  # information on available options.

  # Enable provisioning with a shell script. Additional provisioners such as
  # Puppet, Chef, Ansible, Salt, and Docker are also available. Please see the
  # documentation for more information about their specific syntax and use.
  config.vm.provision "shell", inline: <<-SHELL
    export PRUSTI_DEMO_DIR="/prusti"
    mkdir -p "$PRUSTI_DEMO_DIR"

    apt-get update
    apt-get dist-upgrade -y

    # Install dependencies.
    curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
    apt-get update && apt-get install -y yarn
    apt-get install -y libssl-dev build-essential fish pkg-config
    curl -sL https://deb.nodesource.com/setup_10.x | bash -
    apt-get update && apt-get install -y nodejs

    # Install Docker.
    apt-get install -y \
        apt-transport-https \
        ca-certificates \
        curl \
        gnupg-agent \
        software-properties-common
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -
    apt-key fingerprint 0EBFCD88
    add-apt-repository \
       "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
       $(lsb_release -cs) \
       stable"
    apt-get update
    apt-get install -y docker-ce docker-ce-cli containerd.io

    # 1. Configure Docker.
    # Ensure Docker can control the PID limit
    mount | grep cgroup/pids

    # Ensure Docker can control swap limit.
    # https://docs.docker.com/engine/installation/linux/linux-postinstall/#your-kernel-does-not-support-cgroup-swap-limit-capabilities
    sed -e 's/GRUB_CMDLINE_LINUX=""/GRUB_CMDLINE_LINUX="cgroup_enable=memory swapaccount=1"/g' \
        -i /etc/default/grub
    update-grub

    fallocate -l 1G /swap.fs
    chmod 0600 /swap.fs
    mkswap /swap.fs

    # Set aside disk space.
    fallocate -l 512M /playground.fs
    device=$(losetup -f --show /playground.fs)
    mkfs -t ext3 -m 1 -v $device
    mkdir /mnt/playground
    cat >>/etc/fstab <<EOF
/swap.fs        none            swap   sw       0   0
/playground.fs /mnt/playground  ext3   loop     0   0
EOF

    # Install Rust.
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env

    # 2. Build Prusti
    cd "$PRUSTI_DEMO_DIR"
    git clone /vagrant prusti
    cd prusti
    docker build -t rust-nightly -f docker/playground.Dockerfile .
    export RUSTUP_TOOLCHAIN=$(cat "rust-toolchain")
    rustup toolchain install ${RUSTUP_TOOLCHAIN}
    rustup default ${RUSTUP_TOOLCHAIN}

    # 3. Build `rust-playground`
    cd "$PRUSTI_DEMO_DIR"
    git clone https://github.com/integer32llc/rust-playground.git
    cd rust-playground
    cd ui
    cargo build --release
    cd frontend
    yarn
    yarn run build:production

    # 4. Create service.
    cp /vagrant/docker/playground.service /etc/systemd/system/playground.service
    service playground start
    systemctl enable playground.service

    # 5. Reboot.
    reboot
  SHELL
end
