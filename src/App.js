import { BrowserRouter, Routes, Route } from "react-router-dom";
import Layout from './Layout';
import PlayList from './pages/PlayList';
import Discovery from './pages/Discovery';
import Setting from './pages/Setting';

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<Layout />}>
          <Route index path="/" element={<Discovery />} />
          <Route path="playlist" element={<PlayList />} />
          <Route path="setting" element={<Setting />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}