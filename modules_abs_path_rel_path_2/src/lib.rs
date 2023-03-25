fn serve_order(){               //1:created a serve order function

}
mod back_of_house{              //2:created back of house module which is at same level as of serve order function
    fn delivery(){              //4:created a delivery function which delivers cooked food
        cook();                 //5:calling cooked function which as at same level as of delivery
        super::serve_order();   //6:calling serve order function, but it is at upper level, so we used super here
                                //which take us to crate route and from their we can easily goes to serve order

    }

    fn cook(){                  //3:created cook function which indicated chef cooks food at back house

    }
}