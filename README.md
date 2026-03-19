
- [ Best strategies for Wordle](https://sonorouschocolate.com/notes/index.php?title=The_best_strategies_for_Wordle)
- [Optimal Wordle solutions](https://jonathanolson.net/experiments/optimal-wordle-solutions)

# Xordle Rules

- There are two secret words.
- They share no letters in common.
- You get 8 tries to guess both words.

You start with a clue already given. It's the same for everyone.
The daily puzzle is often themed, but not always, and the theme is often not obvious.

Letters in your guess are:
- 🟩 Green if green in either word (right letter, right spot).
- 🟨 Yellow if yellow in either word (right letter, wrong spot).
- ⬛ Grey if it does not appear in either word.

# 3blue1brown notes

- **probability** of getting a given `Clues` for a `guess`

  $$\text{count of words that match Clues for guess}\over\text{total word count}$$
- **Expected information** of a `guess`

  $$\sum_{\text{clues}} p(\text{clues}) \times (\text{information})$$
- **Information** $$-log_2(p)$$
