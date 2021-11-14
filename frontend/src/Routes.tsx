import Home from './pages/Home'
import About from './pages/About'
import Dashboard from "./pages/Dashboard";
import Logout from "./components/Logout";

export const Routes = [
    {
        path: '/',
        sidebarName: 'Home',
        component: Home
    },
    {
        path: '/about',
        sidebarName: 'About',
        component: About
    },
];


export const ProtectedRoutes = [
    {
        path: '/dashboard',
        sidebarName: 'Dashboard',
        component: Dashboard
    },
    {
        path: '/logout',
        sidebarName: 'Logout',
        component: Logout
    },
];
