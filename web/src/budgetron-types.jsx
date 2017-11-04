import PropTypes from 'prop-types';
import AirbnbPropTypes from 'airbnb-prop-types';

const Transaction = PropTypes.shape({
  date: PropTypes.string.isRequired,
  description: PropTypes.string.isRequired,
  amount: PropTypes.oneOfType([PropTypes.string, PropTypes.number]).isRequired,
  transaction_type: PropTypes.string.isRequired,
  person: PropTypes.string.isRequired,
  original_description: PropTypes.string.isRequired,
  account_name: PropTypes.string.isRequired,
  labels: PropTypes.string.isRequired,
  notes: PropTypes.string.isReqiured,
  transfer_destination_account: PropTypes.string,
  tags: PropTypes.arrayOf(PropTypes.string).isRequired,
});

const RollingBudgetConfig = PropTypes.shape({
  type: PropTypes.oneOf(['RollingBudget']).isRequired,
  split: PropTypes.string.isRequired,
  start_date: PropTypes.string.isRequired,
  amounts: AirbnbPropTypes.valuesOf(PropTypes.string).isRequired,
});

const ReportConfig = PropTypes.oneOfType([
  RollingBudgetConfig,
  PropTypes.shape({
    type: PropTypes.oneOf(['Cashflow', 'Categories']),
  }),
]);

const ReportInfo = PropTypes.shape({
  name: PropTypes.string.isRequired,
  config: ReportConfig.isRequired,
  skip_tags: PropTypes.arrayOf(PropTypes.string),
  only_type: PropTypes.string,
  by_week: PropTypes.bool,
  by_month: PropTypes.bool,
  by_quarter: PropTypes.bool,
  by_year: PropTypes.bool,
});

const RollingBudgetData = PropTypes.shape({
  budgets: AirbnbPropTypes.valuesOf(PropTypes.string).isRequired,
  transactions: PropTypes.arrayOf(PropTypes.string),
});

const CashflowData = PropTypes.shape({
  credit: PropTypes.string.isRequired,
  debit: PropTypes.string.isRequired,
});

const CategoriesData = AirbnbPropTypes.valuesOf(PropTypes.shape({
  amount: PropTypes.string.isRequired,
  transactions: PropTypes.arrayOf(PropTypes.string),
}));

const ReportDataBase = PropTypes.oneOfType([
  RollingBudgetData,
  CashflowData,
  CategoriesData,
]);

const ReportData = PropTypes.oneOfType([
  ReportDataBase,
  PropTypes.shape({
    by_week: AirbnbPropTypes.valuesOf(ReportDataBase),
    by_month: AirbnbPropTypes.valuesOf(ReportDataBase),
    by_quarter: AirbnbPropTypes.valuesOf(ReportDataBase),
    by_year: AirbnbPropTypes.valuesOf(ReportDataBase),
  }),
]);

const Report = PropTypes.shape({
  key: PropTypes.string.isRequired,
  report: ReportInfo.isRequired,
  data: ReportData.isRequired,
});

const BudgetronTypes = {
  Report,
  ReportConfig,
  ReportData,
  ReportDataBase,
  ReportInfo,
  RollingBudgetConfig,
  RollingBudgetData,
  CashflowData,
  CategoriesData,
  Transaction,
};

export default BudgetronTypes;
