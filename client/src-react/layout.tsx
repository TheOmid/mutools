import React from 'react'
import Grid from '@mui/material/Grid';
import Box from '@mui/material/Box';


export const BaseViewGrid = (props: any) => {
    const { Top, Middle, Bottom } = props;
    
    return (
	<Box sx={{ flexGrow: 1, height: '100vh', width: '100vw' }}>
	  <Grid container
                spacing={1}
                direction="column"
                justifyContent="space-evenly"
                alignItems="stretch"
          >

            <Grid item xs={12}>
              <Box sx={{ height: '2vh' }}>
                { Top }
              </Box>
            </Grid>

            <Grid item xs={12}>
              <Box sx={{ height: '38vh' }}>
                { Middle }
              </Box>
            </Grid>

            <Grid item xs={12}>
              <Box sx={{ height: '60vh' }}>
                { Bottom }
              </Box>
            </Grid>

	  </Grid>
	</Box>
    );
};

export const TopbarViewGrid = (props: any) => {
    const { C1, C2, C3, C4, C5 } = props;

    return ( 
        <Box sx={{ flexGrow: 1 }}>
          <Grid container>

            <Grid item xs={1}>
              { C1 }
            </Grid>

            <Grid item xs={1}>
              { C2 }
            </Grid>

            <Grid item xs={1}>
              { C3 }
            </Grid>

            <Grid item xs={1}>
              { C4 }
            </Grid>

            <Grid item xs={5}>
            </Grid>
            
            <Grid item xs={3}>
              { C5 }
            </Grid>            
            
          </Grid>
        </Box>
    );
}
