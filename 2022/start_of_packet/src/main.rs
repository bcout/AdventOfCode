/*
The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the 
    most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would 
    be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that 
    the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the 
    protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four 
    most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer 
        to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a 
    marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all 
    different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been 
    processed.

Here are a few more examples:

    bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
    nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
    nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
    zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

How many characters need to be processed before the first start-of-packet marker is detected?

--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------
--------------------------------------------------------------------------------------------------------

Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

    mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
    bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
    nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
    nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
    zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26

How many characters need to be processed before the first start-of-message marker is detected?
*/


fn check_for_start(group: Vec<u8>) -> bool {
    for i in 0..group.len()-1 {
        if group[i] == group[i+1] {
            return false;
        }
    }
    return true;
}

fn get_group_of_letters(letters: &[u8], start_index: usize, group_size: usize) -> Vec<u8> {
    let mut group: Vec<u8> = vec![0; group_size];
    let mut shift_value;
    let mut index;
    for i in 0..group_size {
        shift_value = (group_size - 1) - i;
        index = start_index - shift_value;
        group[i] = letters[index];
    }
    return group;
    
}

fn main() {
    // Part 1 uses 4, part 2 uses 14
    const START_OF_PACKET_LENGTH: usize = 14;
    let input = include_str!("../input.txt");
    let len = input.len();
    if len < START_OF_PACKET_LENGTH {
        // Line is too short to have a start_of_packet marker, don't even check.
        return;
    }

    let mut index = START_OF_PACKET_LENGTH - 1;
    let characters = input.as_bytes();
    //println!("{:?}", characters);
    while index < len {
        let mut group: Vec<u8> = get_group_of_letters(characters, index, START_OF_PACKET_LENGTH);
        //println!("{:?}", group);
        
        group.sort();
        let is_start_of_packet = check_for_start(group);
        if is_start_of_packet {
            println!("{}", index + 1);
            return;
        }
        

        index = index + 1;
    }
}
