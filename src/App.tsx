import "./App.css";

function App() {
  return (
    <main
      className="min-h-full rounded-lg overflow-hidden bg-zinc-900/30 p-4"
      data-tauri-drag-region
    >
      <h2 className="text-2xl text-white">Press Escape key to quit</h2>
    </main>
  );
}

export default App;
