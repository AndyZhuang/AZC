import React, { useState, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import './App.css';

const DEFAULT_CODE = `# Welcome to AZC Playground!
# AZC is a safe programming language for industrial control.

def greet(name: String) -> String
    "Hello, #{name}!"
end

def main()
    puts greet("World")
    
    # Variables
    let x = 42
    let y = 3.14
    let active = true
    
    puts "x = ", x
    puts "y = ", y
    puts "active = ", active
end
`;

interface Example {
  name: string;
  description: string;
  code: string;
}

function App() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [output, setOutput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [examples, setExamples] = useState<Example[]>([]);
  const [showCCode, setShowCCode] = useState(false);
  const [cCode, setCCode] = useState('');

  // Load examples on mount
  useEffect(() => {
    fetch('/api/examples')
      .then(res => res.json())
      .then(data => setExamples(data))
      .catch(() => {
        // Use default examples if API not available
        setExamples([
          {
            name: "Hello World",
            description: "Basic hello world program",
            code: DEFAULT_CODE
          }
        ]);
      });
  }, []);

  const handleRun = async () => {
    setIsLoading(true);
    setOutput('Compiling...');

    try {
      const response = await fetch('/api/compile', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ code })
      });

      const result = await response.json();

      if (result.success) {
        setOutput(result.output || 'Program completed successfully');
        setCCode(result.c_code || '');
      } else {
        setOutput('Error:\n' + (result.errors || ['Unknown error']).join('\n'));
      }
    } catch (error) {
      setOutput('Failed to connect to server. Is the backend running?');
    }

    setIsLoading(false);
  };

  const handleEditorChange = (value: string | undefined) => {
    if (value !== undefined) {
      setCode(value);
    }
  };

  const loadExample = (example: Example) => {
    setCode(example.code);
    setOutput('');
    setCCode('');
  };

  return (
    <div className="app">
      <header className="header">
        <div className="header-content">
          <h1>
            <span className="logo">AZ</span>
            <span className="title">AZC Playground</span>
          </h1>
          <div className="header-actions">
            <select 
              className="example-select" 
              onChange={(e) => {
                const ex = examples.find(x => x.name === e.target.value);
                if (ex) loadExample(ex);
              }}
            >
              <option value="">Load Example...</option>
              {examples.map(ex => (
                <option key={ex.name} value={ex.name}>{ex.name}</option>
              ))}
            </select>
            <button 
              className="run-button" 
              onClick={handleRun}
              disabled={isLoading}
            >
              {isLoading ? 'Running...' : '▶ Run'}
            </button>
          </div>
        </div>
      </header>

      <main className="main">
        <div className="editor-panel">
          <div className="panel-header">
            <span>Editor</span>
            <span className="lang-badge">.azc</span>
          </div>
          <Editor
            height="100%"
            defaultLanguage="ruby"
            value={code}
            onChange={handleEditorChange}
            theme="vs-dark"
            options={{
              minimap: { enabled: false },
              fontSize: 14,
              lineNumbers: 'on',
              wordWrap: 'on',
              automaticLayout: true,
              tabSize: 2,
            }}
          />
        </div>

        <div className="output-panel">
          <div className="panel-header">
            <span>Output</span>
            <button 
              className="toggle-c-button"
              onClick={() => setShowCCode(!showCCode)}
            >
              {showCCode ? 'Show Output' : 'Show C Code'}
            </button>
          </div>
          <pre className="output">
            {showCCode ? (cCode || '// No C code generated yet') : (output || 'Click "Run" to execute your code')}
          </pre>
        </div>
      </main>

      <footer className="footer">
        <p>
          <a href="https://azc.dev" target="_blank" rel="noopener noreferrer">AZC</a>
          {' '}&bull;{' '}
          <a href="https://github.com/azc-lang/azc" target="_blank" rel="noopener noreferrer">GitHub</a>
          {' '}&bull;{' '}
          <a href="https://azc.dev/docs" target="_blank" rel="noopener noreferrer">Documentation</a>
        </p>
      </footer>
    </div>
  );
}

export default App;