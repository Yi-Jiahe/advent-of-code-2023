import './App.css';
import { useState } from 'react';
import init, { day_1_get_sum_of_calibration_values_in_document, day_1_get_sum_of_calibration_values_in_document_part_2 } from "advent-of-code-2023";

function App() {
  const [answer, setAnswer] = useState();

  const raiseError = () => {
    alert(`Unable to generate solution.

    Please ensure your input is valid. 
                    
    If the problem persists, please submit a bug report with the day and part.`);
  };

  return (
    <div className="App">
      <header className="App-header">
        <textarea id="puzzle-input" />
        <button
          onClick={() => {
            init().then(() => {
              try {
                const puzzle_input = document.getElementById('puzzle-input').value;
                setAnswer(day_1_get_sum_of_calibration_values_in_document(puzzle_input));
              } catch {
                raiseError();
              }
            });
          }}>Part 1</button>
                  <button
          onClick={() => {
            init().then(() => {
              try {
                const puzzle_input = document.getElementById('puzzle-input').value;
                setAnswer(day_1_get_sum_of_calibration_values_in_document_part_2(puzzle_input));
              } catch {
                raiseError();
              }
            });
          }}>Part 2</button>
        <p>{answer}</p>
      </header>
    </div >
  );
}

export default App;
