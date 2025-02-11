use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    let (sender, receiver) = channel();

    let hugevec = vec![0; 1_000_000];
    let mut newvec = vec![];
    let mut handle_vec = vec![];

    for i in 0..10 {
        let sender_clone = sender.clone();
        // work를 넣을 새로운 vec. 1/10 크기입니다.
        let mut work: Vec<u8> = Vec::with_capacity(hugevec.len() / 10);

        // 첫 번째 부분은 0..100_000이 되고 다음 부분은 100_000..200_000이 됩니다.
        work.extend(&hugevec[i * 100_000..(i + 1) * 100_000]);
        
        let handle = spawn(move || {
            // handle을 만듭니다.
            for number in work.iter_mut() {
                // 실제 작업을 수행합니다.
                *number += 1;
            }

            // sender_clone을 사용해 work를 수신자에게 보냅니다.
            sender_clone.send(work).unwrap();
        });

        handle_vec.push(handle);
    }

    for handle in handle_vec {
        // 스레드가 완료될 때까지 대기합니다.
        handle.join().unwrap();
    }

    while let Ok(results) = receiver.try_recv() {
        newvec.push(results); // receiver.recv()의 results를 vec에 넣습니다.
    }

    // 이제 Vec<Vec<u8>>이 생겼습니다.
    // .flatten()을 사용하면 이들을 한 번에 넣을 수 있습니다.
    let newvec = newvec.into_iter().flatten().collect::<Vec<u8>>();
    // 이제 1_000_000개의 u8 숫자가 담긴 하나의 vec입니다.
    println!(
        // 숫자가 모두 1인지 확인하기 위해 몇 가지 숫자를 출력해 봅시다.
        "{:?}, {:?}, total length: {}",
        &newvec[0..10],
        &newvec[newvec.len() - 10..newvec.len()],
        newvec.len() // 길이가 1_000_000개임을 보여 줍니다.
    );

    // 그리고 숫자 하나라도 1이 아니면 패닉을 일으키도록 러스트에 지시합니다.
    assert!(newvec.iter().all(|n| n == &1));
}
