# written

Display disk write total (Linux).

## TO-DO:

* Use box dyn type for all 20 /proc/diskstats fields
* Use From and or FromStr method
* Use [u64; 17] for io_stats fields, or, box type like above
* Add clap support '-r' option for disk read total, consider supporting other fields

## Usage

```sh
$ written
nvme2n1: 20.4 MiB
nvme1n1: 101.9 MiB
nvme0n1: 176.7 MiB
sda: 2.0 MiB
```

## LICENSE

MIT
