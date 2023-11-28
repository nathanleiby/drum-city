ECS

- Entities - a logical agent, like a unit in a game
- Components - various data fields, such as "name", which are associated with entities
- Systems - functions that operate on entities with certain components

Bevy also has
- Resources - globally accessible values. Some are built-in, like `Time`. We can also create our own
- Plugins - wrap common functionality into a single unit. can be toggled on and off

Based on these, how might I structure my game?

- Plugins
  - Sequencer
    - has notes to be played
    - plays sound
    - tracks current location
    - saves user input, to visualize or replay
    - evaluates each input for correctness
  - Input Abstraction?
    - KeyboardInput
    - MidiInput
      - connects device
      - listens for input
  - Songs
    - manages "songs" (could be any sequence of notes)
  - ScoreKeeping
    - tracks performance metrics (accuracy, timing, )
  - Guidance
    - given ScoreKeeping input, suggests what to work on

I'm not yet sure if worth diving into details of ECS yet. But I'll likely be working on Sequencer and Input plugins first, so whatever supports those is top priority.

--

NewType pattern means making a custom type for an existing type, so that typechecking is stricter.
e.g. Age could be `usize` or it could use a newtype of `struct Years(usize)` to ensure we're passing around each type correctly.
This could be especially useful if we're passing inputs to another function where they could get mixed up!
`fn doSomething(years: usize, months: usize)` is made safer in typechecking via `fn doSomething(years: Years, months: Months)`.

--



