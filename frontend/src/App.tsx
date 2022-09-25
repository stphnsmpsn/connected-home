import {Switch, Route} from 'react-router-dom';
import {Routes, ProtectedRoutes} from './Routes';

import NavigationBar from './components/NavigationBar';
import useToken from "./UseToken";
import React from "react";
import Login from "./pages/Login";
import Register from "./pages/Register";


const App: React.FC = () => {
    const {token, setToken} = useToken();

    return (
        <div>
            <NavigationBar/>
            <Switch>
                {Routes.map((route: any) => (
                    <Route exact path={route.path} key={route.path}>
                        <route.component/>
                    </Route>
                ))}
                {token && ProtectedRoutes.map((route: any) => (
                    <Route exact path={route.path} key={route.path}>
                        <route.component token={token}/>
                    </Route>
                ))}
                {!token &&
                    <Route exact path="/login" key="/login">
                        <Login setToken={setToken}/>
                    </Route>
                }
                {!token &&
                    <Route exact path="/register" key="/register">
                        <Register setToken={setToken}/>
                    </Route>
                }
            </Switch>
        </div>
    )
}

export default App;
