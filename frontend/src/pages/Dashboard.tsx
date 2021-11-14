import React from 'react';
import {getCountries} from "../api/countries";

const Dashboard: React.FC = (props: any) => {

    //const [countries, setCountries] = useState<Array<Country>>([]);

    console.log(props);

    const countries = async () => {
        await getCountries(props.token).then((response: any) => {
            console.log(response);
        });
        return (
            <>
            </>
        )
    }

    return (
        <>
            <h1>Dashboard</h1>
            <button onClick={countries}>Click Here</button>
        </>
    );
}

export default Dashboard;
