import React from 'react';
import './App.css';
import { useState } from 'react';
import init, {
  day_1_get_sum_of_calibration_values_in_document, day_1_get_sum_of_calibration_values_in_document_part_2,
  day_2_get_sum_of_possible_game_ids, day_2_get_sum_of_minimum_power,
  day_3_get_sum_of_part_numbers, day_3_get_sum_of_gear_ratios,
  day_4_total_scratchcard_points, day_4_get_final_number_of_cards,
} from "aoc-restore-snow-operations";

function App() {
  const [puzzleInput, setPuzzleInput] = useState("");
  const [answer, setAnswer] = useState("");

  const generateAnswer = (f) => {
    init().then(() => {
      try {
        setAnswer(f(puzzleInput).toString());
      } catch {
        alert(`Unable to generate solution.

        Please ensure your input is valid. 
                        
        If the problem persists, please submit a bug report with the day and part.`);
      }
    });
  };

  const solutions = [
    [day_1_get_sum_of_calibration_values_in_document, day_1_get_sum_of_calibration_values_in_document_part_2],
    [day_2_get_sum_of_possible_game_ids, day_2_get_sum_of_minimum_power],
    [day_3_get_sum_of_part_numbers, day_3_get_sum_of_gear_ratios],
    [day_4_total_scratchcard_points, day_4_get_final_number_of_cards],
  ];

  return (
    <div className="App">
      <h1>Puzzle Input</h1>
      <textarea value={puzzleInput} onChange={(e) => setPuzzleInput(e.target.value)} />
      <h1>Answer</h1>
      <p>{answer}</p>
      <h1>2023</h1>
      <div className='Calendar-Grid'>
        {Array.from(Array(25)).map((_, i) => {
          return (
            <div key={i}>
              <p>Day {i + 1}</p>
              {Array.from(Array(2)).map((_, j) => {
                try {
                  const f = solutions[i][j];
                  return <button onClick={() => { generateAnswer(f) }} key={j}>Part {j + 1}</button>
                } catch {
                  return <button disabled key={j}>Part {j + 1}</button>
                }
              })}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export default App;
