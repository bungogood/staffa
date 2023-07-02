# Staffa
[![Build](../../actions/workflows/build.yaml/badge.svg)](../../actions/workflows/build.yaml)

This project uses gnubg's [position id](https://www.gnu.org/software/gnubg/manual/gnubg.html#gnubg-tech_postionid).

```
Position ID: 4HPwATDgc/ABMA
┌13─14─15─16─17─18─┬───┬19─20─21─22─23─24─┬───┐
│ X           O    │   │ O              X │   │
│ X           O    │   │ O              X │   │
│ X           O    │   │ O                │   │
│ X                │   │ O                │   │
│ X                │   │ O                │   │
│                  │BAR│                  │OFF│
│ O                │   │ X                │   │
│ O                │   │ X                │   │
│ O           X    │   │ X                │   │
│ O           X    │   │ X              O │   │
│ O           X    │   │ X              O │   │
└12─11─10──9──8──7─┴───┴─6──5──4──3──2──1─┴───┘
```

## Move Generation

The move generation follows the algorithm outlined [here](https://bkgm.com/articles/Berliner/BKG-AProgramThatPlaysBackgammon/index.html#sec-III-A). To demonstrate this, we consider the starting position as white and roll (5, 4).

```md
24/20 (4)
- 20/15 -> 24/15
- 13/8 -> 24/20 13/8
- 8/3 -> 24/20 8/3
13/8 (5)
- 13/9 -> 13/9 13/8
- 8/4 -> 13/4 // duplcates
- 6/2 -> 13/8 6/2
13/9 (4)
- 9/4 -> 13/4 // duplcates
- 8/3 -> 13/9 8/3
8/3 (5)
- 6/2 -> 8/3 6/2
8/4 (4)
- 8/3 -> 8/4 8/3
```

## GNU Backgammon

Attached to this project is docker image which runs gnubg, this was required due to issues with installing gnubg on MacOS. Simply run:

```bash
docker build -t gnubg .
docker run --rm -it gnubg
```
 
This allows access to GNU Backgammons command line, simply type `help` for more infromation.

## References
- [GNU Backgammon](https://www.gnu.org/software/gnubg/)
