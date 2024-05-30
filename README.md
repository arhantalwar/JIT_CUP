# JIT CUP

![thumbnail](./thumbnail.png)

## Somewhere in Bellwood 

Ben and Albedo are enjoying their Mr. Smoothy drinks. Once they finish, they are given a choice: they can either give their empty cups to the machine next to them and receive some coins in return, or they can keep the cups. There's a catch: they won't be able to communicate with each other the whole time. The outcome for each depends not only on their own decisions but also on the decisions made by the other.

### Cooperate
If both of them cooperate and give their empty cups, they both receive +2 coins.

![thumbnail](./states/YY.png)

### Cheat
If one of them cheats (i.e., does not give the empty cup) while the other cooperates (i.e., gives the empty cup), the one who cooperates gains +3 coins, and the one who cheats loses 1 coin.

![thumbnail](./states/NY.png)
![thumbnail](./states/YN.png)

If both of them choose not to give the empty cup, neither of them gains any coins.
![thumbnail](./states/NN.png)

## It's Hero Time

It is your time to step into the shoes of ben and make those decisions.
Write a program that will print out either "YES" or "NO" to stdout.
You can write this program in the following languages: C, RUST, PYTHON, JS

## GUIDELINES TO FOLLOW

To participate you must follow the flow.

1) Make a git repo
2) Have the following folder structure

### Folder Structure

```
|──[folder1]/
|  |──helper.c
|──INFO (MANDATORY)
|──main.c (Naming it main is MANDATORY)
```

INFO must contain two things,

YOUR NAME
LANGUAGE YOU USED

## WHAT SHOULD THE CODE LOOK LIKE

There will be 200 rounds played between 2 bots. A single round will be consisted of you getting 2 arguments from the terminal.
These arguments will be the count of current round and the info of what the opponent player had said in the previous round.
In case of the very first round when no one has played any round before the second argument will then say "NONE".

```console
$ ./main CURRENT_ROUND_NUM OPPONENT_PLAYER_PREVIOUS_ROUND_INPUT
```
