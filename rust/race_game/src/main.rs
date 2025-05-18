//! A program to teach myself Rust by implementing a toy video game,
//! inspired by a driving game I played on a lineprinter terminal once,
//! probably coded in BASIC...

use rand::Rng;
use std::io::{Read, StdoutLock, Write};
use std::{thread, time};
use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

const DELAY_MS: u16 = 20;
const DELAY_MS_NO_INPUT: u16 = 250;
const ROAD_LENGTH: u16 = 20;
const ROAD_WIDTH: usize = 10;
const ROAD_PAD: usize = 1;
const INDENT: usize = 50;
const ROAD_ORIGIN_X: u16 = 3;
const ROAD_ORIGIN_Y: u16 = 11;
const START_CAR_X: u16 = 50;
const CAR_Y: u16 = 5;

#[derive(Clone, Copy)]
enum RoadDir {
    Left,
    Right,
    Straight,
}

impl RoadDir {
    fn into(val: u8) -> Self {
        match val {
            1 => RoadDir::Left,
            2 => RoadDir::Right,
            3 => RoadDir::Straight,
            _ => RoadDir::Straight,
        }
    }

    fn edge(self) -> char {
        match self {
            RoadDir::Left => '\\',
            RoadDir::Right => '/',
            RoadDir::Straight => '|',
        }
    }
}

struct RoadBlock {
    indent: usize,
    orientation: RoadDir,
}

impl RoadBlock {
    fn to_string(&self) -> String {
        let s = format!(
            "{}{}{}{}{}",
            String::from_utf8(vec![b' '; self.indent]).unwrap(),
            self.orientation.edge(),
            String::from_utf8(vec![b' '; ROAD_WIDTH]).unwrap(),
            self.orientation.edge(),
            String::from_utf8(vec![b' '; ROAD_PAD]).unwrap()
        );
        s
    }
}

struct RoadRace<'a> {
    indent: usize,
    out: RawTerminal<StdoutLock<'a>>,
    rb: Vec<RoadBlock>,
    carx: u16,
    cary: u16,
    crashes: u16,
}

impl RoadRace<'_> {
    fn reset(mut self, x: u16, y: u16) -> Self {
        self.carx = x;
        self.cary = y;
        self.indent = INDENT;
        self.crashes = 0;

        self.rb.clear();

        for _n in 1..=ROAD_LENGTH {
            self.rb.push(RoadBlock {
                indent: self.indent,
                orientation: RoadDir::Straight,
            });
        }

        self
    }

    fn advance(mut self, upcoming: RoadDir) -> Self {
        let mut num = 0;

        self.rb.pop();

        let block = RoadBlock {
            indent: self.indent,
            orientation: upcoming,
        };

        let block_straight = RoadBlock {
            indent: self.indent,
            orientation: RoadDir::Straight,
        };

        match upcoming {
            RoadDir::Right => {
                self.rb.insert(0, block);
                self.indent += 1
            }
            RoadDir::Straight => {
                self.rb.insert(0, block);
            }
            RoadDir::Left => {
                if self.indent > 0 {
                    self.rb.insert(0, block);
                    self.indent -= 1
                } else {
                    self.rb.insert(0, block_straight);
                }
            }
        }

        for block in self.rb.iter() {
            if num == self.cary {
                // draw car, check for collision
                let road_left_rel = block.indent;
                let road_right_rel = road_left_rel + ROAD_WIDTH;
                let carx_rel: usize = (self.carx + 6).into();
                writeln!(
                    self.out,
                    "{}{}{} @ {} {} {}          ",
                    termion::cursor::Goto(3, num + ROAD_ORIGIN_Y),
                    termion::clear::CurrentLine,
                    block.to_string(),
                    road_left_rel,
                    carx_rel,
                    road_right_rel
                )
                .unwrap();

                let mut car_shape: char = '*';
                if carx_rel > road_left_rel && carx_rel < road_right_rel {
                    car_shape = '#';
                } else {
                    self.crashes += 1;
                }
                write!(
                    self.out,
                    "{}{}",
                    termion::cursor::Goto(8 + self.carx, self.cary + ROAD_ORIGIN_Y),
                    car_shape
                )
                .unwrap();
            } else {
                writeln!(
                    self.out,
                    "{}{}",
                    termion::cursor::Goto(3, num + ROAD_ORIGIN_Y),
                    block.to_string()
                )
                .unwrap();
            }
            num += 1;
        }

        self
    }
}

#[tokio::main]
async fn main() -> Result<(), i32> {
    let mut stdout = std::io::stdout().lock().into_raw_mode().unwrap();
    let mut curr_dir = RoadDir::Straight;

    write!(
        stdout,
        "{}{}'q' to exit. ',' is <Left '.' is Right>{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut race = RoadRace {
        out: stdout,
        rb: Vec::with_capacity(ROAD_LENGTH.into()),
        carx: START_CAR_X,
        cary: CAR_Y,
        indent: INDENT.into(),
        crashes: 0,
    };

    race = race.reset(START_CAR_X, CAR_Y);
    race = race.advance(RoadDir::Straight);

    let mut stdin = async_stdin().bytes();

    for lap in 1..u32::MAX {
        let b = stdin.next();
        let mut moved = true;
        let mut delay = time::Duration::from_millis(DELAY_MS.into());

        //        write!(race.out, "\r{:?}    <- async key             \n\r", b).unwrap();

        match b {
            Some(Ok(b',')) => {
                if race.carx > 0 {
                    race.carx -= 1
                }
            }
            Some(Ok(b'.')) => race.carx += 1,
            Some(Ok(b'q')) => break,
            _ => moved = false,
        }

        if lap % 10 == 0 || moved == true {
            let should_change = rand::thread_rng().gen_range(1..=10);
            if should_change < 5 {
                curr_dir = RoadDir::into(rand::thread_rng().gen_range(1..=3));
            }
            race = race.advance(curr_dir);
        }

        thread::sleep(delay);
    }

    writeln!(race.out, "{}", termion::cursor::Goto(1, 1)).unwrap();
    write!(
        race.out,
        "{} {} crashes\r\n",
        termion::cursor::Show,
        race.crashes
    )
    .unwrap();

    return Ok(());
}
