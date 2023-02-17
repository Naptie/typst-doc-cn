// Test the string methods.
// Ref: false

---
// Test the `len` method.
#test("Hello World!".len(), 12)

---
// Test the `first` and `last` methods.
#test("Hello".first(), "H")
#test("Hello".last(), "o")
#test("🏳️‍🌈A🏳️‍⚧️".first(), "🏳️‍🌈")
#test("🏳️‍🌈A🏳️‍⚧️".last(), "🏳️‍⚧️")

---
// Error: 4-14 string is empty
#{ "".first() }

---
// Error: 4-13 string is empty
#{ "".last() }

---
// Test the `at` method.
#test("Hello".at(1), "e")
#test("Hello".at(4), "o")
#test("Hey: 🏳️‍🌈 there!".at(5), "🏳️‍🌈")

---
// Error: 2-14 string index 2 is not a character boundary
#"🏳️‍🌈".at(2)

---
// Error: 2-15 string index out of bounds (index: 5, len: 5)
#"Hello".at(5)

---
// Test the `slice` method.
#test("abc".slice(1, 2), "b")
#test("abc🏡def".slice(2, 7), "c🏡")
#test("abc🏡def".slice(2, -2), "c🏡d")
#test("abc🏡def".slice(-3, -1), "de")

---
// Error: 2-21 string index -1 is not a character boundary
#"🏳️‍🌈".slice(0, -1)

---
// Test the `contains` method.
#test("abc".contains("b"), true)
#test("b" in "abc", true)
#test("1234f".contains(regex("\d")), true)
#test(regex("\d") in "1234f", true)
#test("abc".contains("d"), false)
#test("1234g" in "1234f", false)
#test("abc".contains(regex("^[abc]$")), false)
#test("abc".contains(regex("^[abc]+$")), true)

---
// Test the `starts-with` and `ends-with` methods.
#test("Typst".starts-with("Ty"), true)
#test("Typst".starts-with(regex("[Tt]ys")), false)
#test("Typst".starts-with("st"), false)
#test("Typst".ends-with("st"), true)
#test("Typst".ends-with(regex("\d*")), true)
#test("Typst".ends-with(regex("\d+")), false)
#test("Typ12".ends-with(regex("\d+")), true)

---
// Test the `find` and `position` methods.
#let date = regex("\d{2}:\d{2}")
#test("Hello World".find("World"), "World")
#test("Hello World".position("World"), 6)
#test("It's 12:13 now".find(date), "12:13")
#test("It's 12:13 now".position(date), 5)

---
// Test the `match` method.
#test("Is there a".match("for this?"), none)
#test(
  "The time of my life.".match(regex("[mit]+e")),
  (start: 4, end: 8, text: "time", captures: ()),
)

// Test the `matches` method.
#test("Hello there".matches("\d"), ())
#test("Day by Day.".matches("Day"), (
  (start: 0, end: 3, text: "Day", captures: ()),
  (start: 7, end: 10, text: "Day", captures: ()),
))

// Compute the sum of all timestamps in the text.
#let timesum(text) = {
  let time = 0
  for match in text.matches(regex("(\d+):(\d+)")) {
    let caps = match.captures
    time += 60 * int(caps.at(0)) + int(caps.at(1))
  }
  str(int(time / 60)) + ":" + str(calc.mod(time, 60))
}

#test(timesum(""), "0:0")
#test(timesum("2:70"), "3:10")
#test(timesum("1:20, 2:10, 0:40"), "4:10")

---
// Test the `replace` method.
#test("ABC".replace("", "-"), "-A-B-C-")
#test("Ok".replace("Ok", "Nope", count: 0), "Ok")
#test("to add?".replace("", "How ", count: 1), "How to add?")
#test("AB C DEF GH J".replace(" ", ",", count: 2), "AB,C,DEF GH J")
#test("Walcemo"
  .replace("o", "k")
  .replace("e", "o")
  .replace("k", "e")
  .replace("a", "e"),
  "Welcome"
)
#test("123".replace(regex("\d$"), "_"), "12_")
#test("123".replace(regex("\d{1,2}$"), "__"), "1__")

---
// Test the `trim` method.
#let str = "Typst, LaTeX, Word, InDesign"
#let array = ("Typst", "LaTeX", "Word", "InDesign")
#test(str.split(",").map(s => s.trim()), array)
#test("".trim(), "")
#test(" abc ".trim(at: start), "abc ")
#test(" abc ".trim(at: end, repeat: true), " abc")
#test("  abc".trim(at: start, repeat: false), "abc")
#test("aabcaa".trim("a", repeat: false), "abca")
#test("aabca".trim("a", at: start), "bca")
#test("aabcaa".trim("a", at: end, repeat: false), "aabca")
#test("".trim(regex(".")), "")
#test("123abc456".trim(regex("\d")), "abc")
#test("123abc456".trim(regex("\d"), repeat: false), "23abc45")
#test("123a4b5c678".trim(regex("\d"), repeat: true), "a4b5c")
#test("123a4b5c678".trim(regex("\d"), repeat: false), "23a4b5c67")
#test("123abc456".trim(regex("\d"), at: start), "abc456")
#test("123abc456".trim(regex("\d"), at: end), "123abc")
#test("123abc456".trim(regex("\d+"), at: end, repeat: false), "123abc")
#test("123abc456".trim(regex("\d{1,2}$"), repeat: false), "123abc4")
#test("hello world".trim(regex(".")), "")

---
// Error: 18-22 expected either `start` or `end`
#{"abc".trim(at: left)}

---
// Test the `split` method.
#test("abc".split(""), ("", "a", "b", "c", ""))
#test("abc".split("b"), ("a", "c"))
#test("a123c".split(regex("\d")), ("a", "", "", "c"))
#test("a123c".split(regex("\d+")), ("a", "c"))
