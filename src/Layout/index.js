import { useState } from 'react';
import { Outlet, NavLink } from 'react-router-dom';
import Footer from './Footer';
import './index.css';

export default ({ children }) => {
  const [playStatus, setPlayStatus] = useState(false);
  const style = ({ isActive }) => ({
    backgroundColor: isActive ? '#d4d4d4' : '#f5f5f5'
  });

  const togglePlayStatus = () => {
    setPlayStatus(!playStatus);
  }

  
  return (
    <div className='page-container'>
      <div className='header-container'>
        <div className='nav-container'>
          <NavLink to="/" style={style} className="link-item">发现</NavLink>
          <NavLink to="/playlist" style={style} className="link-item">图库</NavLink>
          <NavLink to="/setting" style={style} className="link-item">设置</NavLink>
        </div>
      </div>

      <main className='body-container'>
        <Outlet />
      </main>

      <Footer />
    </div>
  );
};