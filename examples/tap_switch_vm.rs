extern crate pg;

use pg::{init, Graph, Brick, Vhost, Tap, Switch, Side};

fn main() {
    // We create 3 vhost interfaces connected together on a switch
    // A tap interface is also added to the switch
    // You can then lanch a virtual machine
    //     -chardev socket,id=char0,path=$SOCKET_PATH
    //     -netdev type=vhost-user,id=mynet,chardev=char0,vhostforce
    //     -device virtio-net-pci,csum=off,gso=off,mac=$MAC,netdev=mynet
    //     -object memory-backend-file,id=mem,size=124M,mem-path=/mnt/huge,share=on

    // Initialize dpdk & stuff
    init();

    // Create some bricks
    let mut tap = Brick::Tap(Tap::new("tap"));
    let mut vh1 = Brick::Vhost(Vhost::new("vhost1", 0).expect("vhost1 creation "));
    let mut vh2 = Brick::Vhost(Vhost::new("vhost2", 0).expect("vhost2 creation "));
    let mut vh3 = Brick::Vhost(Vhost::new("vhost3", 0).expect("vhost3 creation "));
    let mut sw = Brick::Switch(Switch::new("switch", 1, 3, Side::West));

    // Link bricks togather
    tap.link(&mut sw).unwrap();
    sw.link(&mut vh1).unwrap();
    sw.link(&mut vh2).unwrap();
    sw.link(&mut vh3).unwrap();

    // Put every body in a graph
    let mut g = Graph::new("my network");
    g.add(tap).add(sw).add(vh1).add(vh2).add(vh3);

    println!("Now polling packets from tap and vhost interfaces...");
    loop {
        g.poll();
    }
}
