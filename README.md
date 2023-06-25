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

## GNU Backgammon

Attached to this project is docker image which runs gnubg, this was required due to issues with installing gnubg on MacOS. Simply run:

```bash
docker build  -t gnubg .
docker run --rm  -it gnubg
```
 
This allows access to GNU Backgammons command line, simply type `help` for more infromation.

## References
- [GNU Backgammon](https://www.gnu.org/software/gnubg/)
